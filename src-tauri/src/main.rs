// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod file;
mod pilot;
mod serial; // added temporarily (maybe)

use std::fs::File; // addedd for debugging - to be removed/commented when building
use std::io::Write; // same as above
use std::thread;
use std::time::Duration;

use chrono::Utc;
use database::initialize_database; // also temporary
use file::export_to_csv;
use tauri::Manager;

use self::file::*;
use self::pilot::*;
use self::serial::*;

/// tauri command that calls the backend (rust) export_to_csv function
#[tauri::command]
fn export_csv_command(csv_path: String) -> Result<String, String> {
    let conn =
        initialize_database().map_err(|e| format!("Failed to initialize database: {}", e))?;
    export_to_csv(&conn, &csv_path).map_err(|e| e.to_string())?;
    Ok("CSV export successful".to_string())
}

///tauri command to get the project directory path or a parent directory's path (unused)
#[tauri::command]
fn get_project_dir(steps: usize) -> Result<String, String> {
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    let mut path = std::path::PathBuf::from(current_dir);

    for _ in 0..steps {
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            return Err("Reached the root directory. Cannot go up further.".to_string());
        }
    }

    path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to convert path to string.".to_string())
}

fn main() {
    // Initialize the database
    let conn = initialize_database().expect("Failed to initialize database");

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            thread::spawn(move || {
                loop {
                    let battery_bench = BatteryBench {
                        id: 0,
                        port: "COM4".to_string(),
                        temperature: 2020,
                        battery_temperature: 2013,
                        electronic_load_temperature: 2054,
                        voltage: 534,
                        current: 324,
                        state: BatteryBenchState::Standby,
                        status: CompletionStatus::InProgress,
                        start_date: Utc::now(),
                        end_date: Utc::now(),
                    };
                    //TODO: update data by communicating with the battery

                    // Log battery data
                    if let Err(e) = log_battery(&conn, battery_bench.clone()) {
                        eprintln!("Failed to log battery data: {}", e);
                    }

                    // Emit battery data to frontend
                    app_handle
                        .emit_all("display-battery", battery_bench)
                        .unwrap();

                    thread::sleep(Duration::from_secs(1)); // Adjust the interval as needed
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            export_csv_command,
            get_project_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
