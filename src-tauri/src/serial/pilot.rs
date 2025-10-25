use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{ipc::Channel, State};

use crate::{
    database::{models::BatteryLog, sqlite},
    serial::serial::{BatteryCommand, Command},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Type)]
pub enum BatteryState {
    #[default]
    Standby,
    Charge,
    Discharge,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct Battery {
    id: u8,
    state: BatteryState,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct Bench {
    batteries: Vec<Battery>,
    port: String,
}

impl Bench {
    pub fn init_searching() {
        todo!()

        //spawns a thread that scans all the open ports every sec and adds them to open ports list
    }

    pub fn new() -> Result<BatteryLog, &'static str> {
        todo!()

        //pings a newly open port to check if firmware is running on the port
    }

    //starts a thread for the thread that pings the bench every sec
    pub fn start_sequence(
        &self,
        state: State<'_, Mutex<crate::state::AppState>>,
        on_event: Channel<BatteryLog>,
    ) {
        let bench = Arc::new(Mutex::new(self.clone()));
        let state_arc = state.inner().clone();

        // thread::spawn(move || loop {
        //     let mut bench_guard = bench.lock().unwrap();
        //     bench_guard.complete_sequence_step(state.clone(), on_event.clone());
        // });
    }

    pub fn complete_sequence_step(
        &mut self,
        state: State<'_, Mutex<crate::state::AppState>>,
        on_event: Channel<BatteryLog>,
    ) {
        let mut bat_count = 0;

        for battery in &self.batteries {
            // request data
            match data_request(self.clone(), battery.clone()) {
                Ok(data) => {
                    sqlite::insert_battery_log(state.clone(), data.clone());
                    // pass to channel
                    on_event.send(data).unwrap();
                }
                Err(error) => print!("Error while fetching data: {}", error),
            }

            bat_count += 1;
        }

        while bat_count < 4 {
            // ping with new ID
            match assign_id(self.clone()) {
                Ok(id) => {
                    self.batteries.push(Battery {
                        id: id,
                        state: BatteryState::Standby,
                    });
                    bat_count += 1;
                }
                Err(_) => todo!(),
            }
        }
    }

    pub fn complete_sequence(&mut self) {
        todo!()
    }
}

#[tauri::command]
#[specta::specta]
pub fn set_state(
    bench: Bench,
    battery: Battery,
    new_state: BatteryState,
) -> Result<String, String> {
    let command = match new_state {
        BatteryState::Standby => Command::SetStandBy,
        BatteryState::Charge => Command::SetCharge,
        BatteryState::Discharge => Command::SetDischarge,
    };

    let battery_cmd = BatteryCommand {
        command: command,
        battery_id: battery.id,
        payload: vec![],
    };
    let encoded_data = battery_cmd.encode();

    let mut response = vec![0u8; command.response_lenght()];

    let mut port = serialport::new(bench.port, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| e.to_string())?;

    port.write_all(&encoded_data).map_err(|e| e.to_string())?;

    loop {
        match port.read_exact(&mut response) {
            Ok(_) => break,
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("State change command not ready");
                thread::sleep(Duration::from_millis(333));
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    dbg!(&response);

    match BatteryCommand::decode(&response) {
        Ok(decoded_response) => {
            // Check if the response command matches what we sent
            if decoded_response.command == command && decoded_response.battery_id == battery.id {
                Ok(format!(
                    "Battery {} state successfully changed to {:?}",
                    battery.id, new_state
                ))
            } else {
                Err(format!(
                    "Unexpected response: got {:?} for battery {}, expected {:?} for battery {}",
                    decoded_response.command, decoded_response.battery_id, command, battery.id
                ))
            }
        }
        Err(error) => Err(error),
    }
}

#[tauri::command]
#[specta::specta]
pub fn assign_id(bench: Bench) -> Result<u8, String> {
    let command = Command::AssignId;
    let mut battery_id: u8 = 0;

    // FIXME: potentially infinite loop
    while bench
        .batteries
        .iter()
        .any(|battery| battery.id == battery_id)
    {
        if battery_id == 255 {
            battery_id = 0;
        }
        battery_id += 1;
    }

    let battery_cmd = BatteryCommand {
        command: command,
        battery_id: battery_id,
        payload: vec![],
    };
    let encoded_data = battery_cmd.encode();

    let mut response = vec![0u8; command.response_lenght()];

    let mut port = serialport::new(bench.port, 9600)
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

    match BatteryCommand::decode(&response) {
        Ok(decoded_response) => battery_cmd.parse_assign_id(&decoded_response.payload),
        Err(error) => Err(error),
    }
}

#[tauri::command]
#[specta::specta]
pub fn data_request(bench: Bench, battery: Battery) -> Result<BatteryLog, String> {
    let command = Command::RequestData;
    let battery_cmd = BatteryCommand {
        command: command,
        battery_id: battery.id,
        payload: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    let encoded_data = battery_cmd.encode();

    let mut response = vec![0u8; command.response_lenght()];

    let mut port = serialport::new(bench.port.clone(), 9600)
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

    match BatteryCommand::decode(&response) {
        Ok(decoded_response) => {
            battery_cmd.parse_request_data(&decoded_response.payload, battery.id, bench.port)
        }
        Err(error) => Err(error),
    }
}

pub fn get_current_time() -> String {
    Utc::now().to_rfc3339()
}
