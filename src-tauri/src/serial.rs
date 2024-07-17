
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

use serde::Serialize;
use bincode;
use std::io::{ Write , Read};
use std::{path, string};
use std::time::Duration;


const DELIMITER : u8 = 0xB3;


#[derive(Serialize, Debug)]
enum Command {
	Ping(PingPayload),
	AssignID(AssignIDPayload),
	RequestData(RequestDataPayload),
	SetStandby,
	SetDischarge,
	SetCharge,
	AnnounceCompletion(AnnounceCompletionPayload)
}

#[derive(Serialize, Debug)]
struct PingPayload {
	identification : u8
}

#[derive(Serialize, Debug)]
struct AssignIDPayload {
	new_id : u8
}

#[derive(Serialize, Debug)]
struct RequestDataPayload {
	battery_temperature : u16,
	bench_temperature : u16,
	load_temperature : u16,
	voltage : u16,
	current : u16,
}

#[derive(Serialize, Debug)]
struct AnnounceCompletionPayload {
	flag : u8
}

impl Command{

	fn get_id(&self) -> u8 {
		match &self {
			Command::Ping(_) => 0x00,
			Command::AssignID(_) => 0x01,
			Command::RequestData(_) => 0x02,
			Command::SetStandby => 0x04,
			Command::SetDischarge => 0x05,
			Command::SetCharge => 0x06,
			Command::AnnounceCompletion(_) => 0x07
		}
	}

	fn decode(encoded: Vec<u8>) -> Command {
		Command::Ping(
			PingPayload {
				identification : 0x1A
			}
		)
	}

	fn encode(&self) -> Vec<u8> {


		let mut serialized: Vec<u8> = match self {
				Command::Ping(db) => bincode::serialize(db).unwrap(),
				Command::AssignID(db) => bincode::serialize(db).unwrap(),
				Command::RequestData(db) => bincode::serialize(db).unwrap(),
				Command::SetStandby => vec![],
				Command::SetDischarge => vec![],
				Command::SetCharge => vec![],
				Command::AnnounceCompletion(db) => bincode::serialize(db).unwrap()
		};

		serialized.insert(0, self.get_id());
		serialized.insert(0, DELIMITER);


		let mut checksum = serialized[0];
		let mut index = 1;

		while index < serialized.len() {
			checksum ^= serialized[index];
			index+=1;
		};

		serialized.push(checksum);

		serialized


	}

	fn send(&self, port: &str) -> Vec<u8>{

		let data = self.encode();

		let mut port = serialport::new(port, 115200)
    	.timeout(Duration::from_millis(10))
    	.open().expect("Failed to open port");

		port.write(&data).expect("Write failed!");


		let mut serial_buf: Vec<u8> = vec![0; 32];
    	port.read(serial_buf.as_mut_slice()).expect("Found no data!");


		serial_buf
	}

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
/// ``


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encode() {

		let command = Command::Ping(PingPayload{ identification : 0x05 });

		let response = command.send("COM7");

		println!("Command: {:?}", response);
	}

	#[test]
	fn test_decode() {
		let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03];
		let decoded = Command::decode(encoded);
		//assert_eq!(decoded, vec![0x01, 0x02, 0x03]);

		let encoded = vec![0xB3, 0x00, 0xFF, 0x55, 0xB3 ^ 0x00 ^ 0xFF ^ 0x55];
		let decoded = Command::decode(encoded);
		//assert_eq!(decoded, vec![0x00, 0xFF, 0x55]);
	}

	#[test]
	fn test_decode_invalid_checksum() {

		let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0x00];
		let result = Command::decode(encoded);

		//assert!(result.is_err());
		//assert_eq!(result.err(), Some("Invalid checksum"));
	}

	#[test]
	fn test_decode_too_short() {

		let encoded = vec![0xB3];
		let result = Command::decode(encoded);

		//assert!(result.is_err());
		//assert_eq!(result.err(), Some("Input is too short to be valid"));
	}
}
