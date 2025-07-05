use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use crate::database::sqlite::get_all_battery_logs;
use crate::database::{models::BatteryLog, sqlite::establish_connection};
use crate::state::AppState;

use csv::Writer;
use diesel::prelude::*;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub fn export_csv(statee: State<'_, Mutex<AppState>>, base_path: String) -> Result<(), String> {
    let mut app_state = statee.lock().map_err(|e| e.to_string())?;

    let connection = match &mut app_state.db_connection {
        Some(conn) => conn,
        None => {
            let conn = establish_connection(&app_state.db_path).map_err(|e| e.to_string())?;
            app_state.db_connection = Some(conn);
            app_state.db_connection.as_mut().unwrap()
        }
    };

    let all_logs = get_all_battery_logs(connection)?;

    let mut grouped_logs: std::collections::HashMap<i32, Vec<BatteryLog>> =
        std::collections::HashMap::new();

    for log in all_logs {
        grouped_logs.entry(log.id).or_default().push(log);
    }

    std::fs::create_dir_all(&base_path).map_err(|e| e.to_string())?;

    for (battery_id, logs) in grouped_logs {
        let file_path = Path::new(&base_path).join(format!("battery_{}.csv", battery_id));
        let mut wtr = Writer::from_path(&file_path).map_err(|e| e.to_string())?;

        for log in logs {
            wtr.serialize(log).map_err(|e| e.to_string())?;
        }

        wtr.flush().map_err(|e| e.to_string())?;
    }

    Ok(())
}
