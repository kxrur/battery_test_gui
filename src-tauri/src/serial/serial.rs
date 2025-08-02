use crc::Crc;
use serde::{Deserialize, Serialize};
use serialport::available_ports;
use specta::Type;
use std::io::{Read, Write};
use std::time::Duration;
use std::{thread, vec};

const DELIMITER: u8 = 0xB3;

const CRC8_AUTOSAR: Crc<u8> = Crc::<u8>::new(&crc::CRC_8_AUTOSAR);

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize)]
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
            Command::RequestData => 12,
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

    pub fn decode(packet: &[u8]) -> Option<(Command, &[u8])> {
        if packet.len() < 3 || packet[0] != DELIMITER {
            return None;
        }

        let command_id = packet[1];
        let received_crc = *packet.last()?;
        let payload = &packet[1..packet.len() - 1];
        let calculated_crc = CRC8_AUTOSAR.checksum(payload);

        if calculated_crc != received_crc {
            return None;
        }

        let command = match command_id {
            0x00 => Command::Ping,
            0x01 => Command::AssignId,
            0x02 => Command::RequestData,
            0x04 => Command::SetCharge,
            0x05 => Command::SetDischarge,
            0x06 => Command::SetStandBy,
            0x07 => Command::RequestCompletion,
            _ => return None,
        };

        Some((command, &packet[2..packet.len() - 1]))
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
    if let Some((command, payload)) = decoded_data {
        println!("Command: {:?}", command);
        println!("Payload: [{}]", format_hex(payload));
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
    Ok(response)
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
    fn test_decode() {}

    #[test]
    fn test_decode_invalid_checksum() {}

    #[test]
    fn test_decode_too_short() {}
}
