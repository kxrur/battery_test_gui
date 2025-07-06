use std::{sync::Mutex, thread, time};
use tauri::{ipc::Channel, AppHandle, Manager};

use specta_typescript::Typescript;
use tauri_specta::*;

mod database;

mod state;

use database::export::export_csv;
use database::sqlite::init_database;

use crate::{
    database::{models::BatteryLog, sqlite::insert_battery_log},
    state::AppState,
};

#[tauri::command]
#[specta::specta]
async fn parseLog(app: AppHandle, on_event: Channel<BatteryLog>) {
    thread::spawn(move || loop {
        let log = BatteryLog {
            record_id: Some(32),
            id: 3,
            port: "port".to_string(),
            temperature: 31,
            battery_temperature: 22,
            electronic_load_temperature: 12,
            voltage: 300,
            current: 500,
            state: "state".to_string(),
            status: "status".to_string(),
            start_date: Some("start date".to_string()),
            end_date: Some("end date".to_string()),
        };
        thread::sleep(time::Duration::from_secs(2));
        on_event.send(log).unwrap();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        insert_battery_log,
        export_csv,
        parseLog
    ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            app.manage(Mutex::new(AppState::default()));

            builder.mount_events(app);

            init_database(app.app_handle())?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
