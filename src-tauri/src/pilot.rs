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

	pub fn init_searching() {
		todo!()

		//spawns a thread that scans all the open ports every sec and adds them to open ports list
	}

	pub fn new() -> Result<BatteryBench, &'static str> {
		todo!()

		//pings a newly open port to check if firmware is running on the port
	}
	
	pub fn start_sequence(&mut self) {
		todo!()

		//starts a thread for the thread that pings the bench every sec
	}
	

	pub fn complete_sequence_step(&mut self) {
		todo!()
	}
	
	pub fn complete_sequence(&mut self) {
		todo!()
	}
}
