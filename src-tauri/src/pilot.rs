use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Default, Serialize, Clone)]
pub enum BatteryBenchState {
	#[default]
	Standby,
	Charge,
	Discharge
}

#[derive(Debug, Serialize, Clone)]
pub enum CompletionStatus {
	Success,
	Fail,
	InProgress
}


#[derive(Debug, Serialize, Clone)]
pub struct BatteryBench {
	pub id: u8,
	pub port: String,
	pub temperature: u16,
	pub battery_temperature: u16,
	pub electronic_load_temperature: u16,
	pub voltage: u16,
	pub current: u16,
	pub state: BatteryBenchState,
	pub status: CompletionStatus,
	pub start_date: DateTime<Utc>,
	pub end_date: DateTime<Utc>,
}

impl BatteryBench {
	pub fn new() -> Result<BatteryBench, &'static str> {
		todo!()
	}
	
	pub fn start_sequence(&mut self) {
		todo!()
	}
	
	pub fn complete_sequence_step(&mut self) {
		todo!()
	}
	
	pub fn complete_sequence(&mut self) {
		todo!()
	}
}


#[derive(Debug)]
pub struct Port{
	addr : String,
	state : PortStatus,
}	

#[derive(Debug)]
pub enum PortStatus {
	Pinged,
	NoResponse,
	Unpinged,
}


impl Port {
	pub fn init() -> Vec<String> {

		todo!()	
		//Spawn a thread that checks for open ports to ping and 
		//check if they are battery testers

	}
}