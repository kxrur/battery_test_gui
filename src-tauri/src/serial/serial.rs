use serde::{Deserialize, Serialize};
use serialport::available_ports;
use specta::Type;
use std::io::{Read, Write};
use std::time::Duration;
use std::{thread, vec};

const DELIMITER: u8 = 0xB3;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize)]
pub enum Command {
    Ping = 0x00,
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
    fn id(&self) -> u8 {
        *self as u8
    }

    fn response_lenght(&self) -> usize {
        match self {
            Command::RequestData => 12,
            Command::RequestCompletion => 4,
            _ => 3,
        }
    }

    fn encode(&self) -> Vec<u8> {
        vec![DELIMITER, self.id(), DELIMITER ^ self.id()]
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
    let encoded_data = value.encode();
    dbg!(&encoded_data);

    let expected_bytes = value.response_lenght();
    dbg!(&expected_bytes);

    thread::sleep(Duration::from_secs(15));

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

/// Encodes a slice of bytes by prepending 0xB3 and appending a checksum.
///
/// The checksum is calculated as the XOR of every byte in the resulting slice, including the prepended 0xB3.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to be encoded.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded bytes.
///
/// # Example
///
/// ```
/// let data = vec![0x01, 0x02, 0x03];
/// let encoded = [0xB3, 0x00, 0x05, 0xB3 ^ 0x00 ^ 0x05];
/// assert_eq!(encoded, vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03]);
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encode() {
        let response = command_request(Command::RequestCompletion, "COM7").await;

        println!("{:?}", response);
    }

    #[test]
    fn test_decode() {}

    #[test]
    fn test_decode_invalid_checksum() {}

    #[test]
    fn test_decode_too_short() {}
}
