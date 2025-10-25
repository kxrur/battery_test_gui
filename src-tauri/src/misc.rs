use std::sync::Mutex;

use chrono::{Duration, Utc};
use rand::{distr::Alphanumeric, Rng};
use tauri::State;

use crate::{
    database::models::{BatteryLog, Test},
    database::sqlite::{insert_battery_log, insert_test},
    state::AppState,
};

#[tauri::command]
#[specta::specta]
pub async fn populate_fake_data(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    for test_index in 0..10 {
        let test_name = format!("Test_{}", random_string(5));
        let start_date = Utc::now()
            .checked_sub_signed(Duration::days(test_index))
            .unwrap()
            .to_rfc3339();

        let test = Test {
            test_id: None,
            test_name,
            start_date,
        };

        let inserted_test = insert_test(state.clone(), test)?;

        for i in 0..4 {
            for j in 0..10 {
                let now = Utc::now().naive_utc();
                let log = BatteryLog {
                    record_id: None,
                    id: (i) as i32,
                    port: format!("COM{}", rand::rng().random_range(1..=10)),
                    battery_temperature: rand::rng().random_range(25..=50),
                    bench_temperature_mosfet: rand::rng().random_range(20..=40),
                    bench_temperature_resistor: rand::rng().random_range(20..=40),
                    load: rand::rng().random_range(30..=60),
                    voltage: rand::rng().random_range(3000..=4200),
                    current: rand::rng().random_range(100..=1000),
                    state: random_state(),
                    status: random_status(),
                    start_date: Some(now.to_string()),
                    end_date: None,
                    test_id: inserted_test.test_id.unwrap(),
                };

                insert_battery_log(state.clone(), log)?;
            }
        }
    }

    Ok(())
}

fn random_string(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn random_state() -> String {
    let states = ["idle", "charging", "discharging", "running"];
    states[rand::rng().random_range(0..states.len())].to_string()
}

fn random_status() -> String {
    let statuses = ["ok", "warning", "error"];
    statuses[rand::rng().random_range(0..statuses.len())].to_string()
}
