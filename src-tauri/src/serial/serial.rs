use serde::{Deserialize, Serialize};
use specta::Type;
/// Decodes a slice of bytes by removing the prepended 0xB3 and verifying the checksum.
///
/// The checksum is verified as the XOR of every byte in the input slice. If the checksum does not match, the function will return an error.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to be decoded. It must have at least 2 bytes: the prepended 0xB3 and the checksum.
///
/// # Returns
///
/// A `Result<Vec<u8>, &'static str>` containing the decoded bytes or an error message if the checksum is invalid.
///
/// # Example
///
/// ```
/// let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03];
/// let decoded = decode(&encoded).unwrap();
/// assert_eq!(decoded, vec![0x01, 0x02, 0x03]);
/// ```
use std::io::{Read, Write};
use std::time::Duration;
use std::{thread, vec};

const DELIMITER: u8 = 0xB3;

#[derive(Debug, Clone, Copy, Type)]
enum Command {
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
fn command_request(value: Command, port_num: &str) -> Vec<u8> {
    let encoded_data = value.encode();

    let expected_bytes = value.response_lenght();

    let mut response: Vec<u8> = vec![0; expected_bytes];

    let mut port = serialport::new(port_num, 19200)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    port.write_all(&encoded_data).expect("Write failed!");

    //change me pls*****************************************************
    let mut _i = 0;
    let mut _has_result = true;

    while port.read_exact(response.as_mut_slice()).is_err() {
        thread::sleep(Duration::from_millis(333));
        println!("data was not ready");
    }

    response
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

    #[test]
    fn test_encode() {
        let response = command_request(Command::RequestCompletion, "COM7");

        println!("{:?}", response);
    }

    #[test]
    fn test_decode() {}

    #[test]
    fn test_decode_invalid_checksum() {}

    #[test]
    fn test_decode_too_short() {}
}
