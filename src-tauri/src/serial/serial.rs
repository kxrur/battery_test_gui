use crc::Crc;
use serde::{Deserialize, Serialize};
use serialport::available_ports;
use specta::Type;
use std::io::{Read, Write};
use std::time::Duration;
use std::{thread, vec};

use crate::database::models::BatteryLog;

const DELIMITER: u8 = 0xB3;

const CRC8_AUTOSAR: Crc<u8> = Crc::<u8>::new(&crc::CRC_8_AUTOSAR);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
pub enum Command {
    Ping = 0x00,
    AssignId = 0x01,
    RequestData = 0x02,
    SetCharge = 0x04,
    SetDischarge = 0x05,
    SetStandBy = 0x06,
    RequestCompletion = 0x07,
}

#[derive(Debug)]
struct PingPayload {
    bench_status: u8,
}

#[derive(Debug)]
struct AnnounceCompletionPayload {
    bench_status: u8,
    experiment_status: u8,
}

impl Command {
    fn id(&self) -> [u8; 2] {
        [DELIMITER, *self as u8]
    }

    fn response_lenght(&self) -> usize {
        match self {
            Command::RequestData => 13,
            Command::Ping => 4,
            Command::AssignId => 4,
            Command::RequestCompletion => 4,
            _ => 3,
        }
    }

    fn checksum(id: &[u8], payload: &[u8]) -> u8 {
        let mut digest = CRC8_AUTOSAR.digest();
        digest.update(id);
        digest.update(payload);
        digest.finalize()
    }

    fn encode(&self, payload: &[u8]) -> Vec<u8> {
        let id = self.id();
        let mut buffer = Vec::with_capacity(id.len() + payload.len() + 1);
        buffer.extend_from_slice(&id);
        buffer.extend_from_slice(payload);
        buffer.push(Self::checksum(&id, payload));
        buffer
    }

    pub fn decode(packet: &[u8]) -> Result<(Command, &[u8]), String> {
        if packet.len() < 3 {
            return Err("Packet too short".to_string());
        }

        let command_id = packet[1];
        let received_crc = *packet.last().ok_or("Missing CRC")?;

        let id = [packet[0], packet[1]];
        let payload = &packet[2..packet.len() - 1];
        let calculated_crc = Self::checksum(&id, payload);

        if calculated_crc != received_crc {
            return Err(format!(
                "Invalid CRC: expected {calculated_crc}, got {received_crc}"
            ));
        }

        let command = match command_id {
            0x00 => Command::Ping,
            0x01 => Command::AssignId,
            0x02 => Command::RequestData,
            0x04 => Command::SetCharge,
            0x05 => Command::SetDischarge,
            0x06 => Command::SetStandBy,
            0x07 => Command::RequestCompletion,
            _ => return Err(format!("Unknown command ID: {command_id}")),
        };

        Ok((command, payload))
    }
    pub fn parse_ping_payload(&self, payload: &[u8]) -> Result<PingPayload, String> {
        if *self != Command::Ping {
            return Err("parse_ping_payload called on wrong command".into());
        }
        if payload.len() != 1 {
            return Err("Invalid Ping payload length".into());
        }
        Ok(PingPayload {
            bench_status: payload[0],
        })
    }

    pub fn parse_assign_id(&self, payload: &[u8]) -> Result<u8, String> {
        if *self != Command::AssignId {
            return Err("parse_assign_id called on wrong command".into());
        }
        if payload.len() != 1 {
            return Err("Invalid AssignId payload length".into());
        }
        Ok(payload[0])
    }

    pub fn parse_request_data(&self, payload: &[u8]) -> Result<BatteryLog, String> {
        if *self != Command::RequestData {
            return Err("parse_request_data called on wrong command".into());
        }
        if payload.len() != 11 {
            return Err("Invalid RequestData payload length".into());
        }

        let battery_temperature = i16::from_be_bytes([payload[0], payload[1]]) as f32 / 100.0;
        let bench_temperature = i16::from_be_bytes([payload[2], payload[3]]) as f32 / 100.0;
        let load_temperature = i16::from_be_bytes([payload[4], payload[5]]) as f32 / 100.0;
        let voltage = i16::from_be_bytes([payload[6], payload[7]]) as i32;
        let current = i16::from_be_bytes([payload[8], payload[9]]) as i32;

        Ok(BatteryLog {
            record_id: None,
            id: 0, //FIXME:
            port: String::new(),
            temperature: bench_temperature as i32,
            battery_temperature: battery_temperature as i32,
            electronic_load_temperature: load_temperature as i32,
            voltage,
            current,
            state: String::new(),
            status: String::new(),
            start_date: None,
            end_date: None,
            test_id: 0,
        })
    }

    pub fn parse_completion(&self, payload: &[u8]) -> Result<AnnounceCompletionPayload, String> {
        if *self != Command::RequestCompletion {
            return Err("parse_completion called on wrong command".into());
        }
        if payload.len() != 1 {
            return Err("Invalid Completion payload length".into());
        }
        let flags = payload[0];
        Ok(AnnounceCompletionPayload {
            bench_status: flags,
            experiment_status: flags,
        })
    }
}

struct CompletionStatus {
    bench_status: u8,
    experiment_status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestDataPayload {
    battery_temperature: u16,
    bench_temperature: u16,
    load_temperature: u16,
    voltage: u16,
    current: u16,
}

#[tauri::command]
#[specta::specta]
pub fn detect_serial_ports() -> Result<Vec<String>, String> {
    match available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|p| p.port_name).collect()),
        Err(e) => Err(format!("Error listing serial ports: {}", e)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn command_request(value: Command, port_num: &str) -> Result<Vec<u8>, String> {
    let encoded_data = value.encode(&[0x3B]);
    println!("Encoded: [{}]", format_hex(&encoded_data));

    let expected_bytes = value.response_lenght();
    println!("Expected Bytes: {}", expected_bytes);

    let decoded_data = Command::decode(&encoded_data);
    match decoded_data {
        Ok((command, payload)) => {
            println!("Command: {:?}", command);
            println!("Payload: [{}]", format_hex(payload));
        }
        Err(err) => {
            eprintln!("Decode error: {}", err);
        }
    }

    let mut response = vec![0u8; expected_bytes];

    let mut port = serialport::new(port_num, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| e.to_string())?;

    port.write_all(&encoded_data).map_err(|e| e.to_string())?;

    loop {
        match port.read_exact(&mut response) {
            Ok(_) => break,
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("data was not ready");
                thread::sleep(Duration::from_millis(333));
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    dbg!(&response);

    if response.len() < 3 || response[0] != DELIMITER {
        Err("invalid response".to_string())
    } else {
        Ok(response)
    }
}

fn format_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{:02X}", b))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc8_autosar_vector() {
        // use https://crccalc.com to compute the expected values
        let data = [0x12, 0x34, 0x56];
        let crc = CRC8_AUTOSAR.checksum(&data);
        assert_eq!(crc, 0x7A, "CRC-8 AUTOSAR failed!");

        let data = [0x38, 0x46, 0x53, 0x82];
        let crc = CRC8_AUTOSAR.checksum(&data);
        assert_eq!(crc, 0x7E, "CRC-8 AUTOSAR failed!");
    }

    #[test]
    fn test_crc8_autosar_digest_vector() {
        let data = [0x12, 0x34];
        let mut digest = CRC8_AUTOSAR.digest();
        digest.update(&data);
        let data_2 = [0x56];
        digest.update(&data_2);
        let crc = digest.finalize();
        assert_eq!(crc, 0x7A, "CRC-8 AUTOSAR failed for first input!");

        let data = [0x38, 0x46, 0x53];
        let mut digest = CRC8_AUTOSAR.digest();
        digest.update(&data);
        let data_2 = [0x82];
        digest.update(&data_2);
        let crc = digest.finalize();
        assert_eq!(crc, 0x7E, "CRC-8 AUTOSAR failed for second input!");
    }

    #[test]
    fn test_encode() {
        let data = &[0x12, 0x34, 0x56];
        let cmd: Command = { Command::Ping };
        let encoded_cmd = cmd.encode(data);

        let mut expected = Vec::new();
        expected.extend_from_slice(&cmd.id());
        expected.extend_from_slice(data);
        // use https://crccalc.com to compute the expected values
        expected.push(0x17);

        println!("left : {:02X?}", encoded_cmd);
        println!("right: {:02X?}", expected);

        assert_eq!(encoded_cmd, expected, "command encode failed")
    }

    #[test]
    fn test_decode() {
        let cmd = Command::Ping;
        let pld = &[0x01];
        let packet = &cmd.encode(pld);
        let (command, payload) = Command::decode(packet).expect("decode failed");

        assert_eq!(command.id(), cmd.id());
        assert_eq!(payload, pld);

        let cmd = Command::RequestData;
        let pld = &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
        let packet = &cmd.encode(pld);
        let (command, payload) = Command::decode(packet).expect("decode failed");

        assert_eq!(command.id(), cmd.id());
        assert_eq!(payload, pld);
    }

    #[test]
    fn test_decode_invalid_checksum() {}

    #[test]
    fn test_decode_too_short() {}
}
