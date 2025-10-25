use std::{sync::Mutex, thread, time};
use tauri::{ipc::Channel, Manager};

use specta_typescript::Typescript;
use tauri_specta::*;

use rand::Rng;

mod database;
mod serial;

mod misc;
mod state;

use database::export::export_csv;
use database::sqlite::init_database;
use misc::populate_fake_data;

use crate::{
    database::{
        models::BatteryLog,
        sqlite::{
            delete_test, get_all_battery_logs, get_all_tests, get_battery_logs_for_test,
            insert_battery_log, insert_new_test, insert_test,
        },
    },
    serial::{
        pilot::{assign_id, data_request, set_state},
        serial::{command_request, detect_serial_ports},
    },
    state::AppState,
};

#[tauri::command]
#[specta::specta]
async fn parse_log(on_event: Channel<BatteryLog>) {
    thread::spawn(move || loop {
        let mut rng = rand::rng();

        let temp: i32 = rng.random();
        let log = BatteryLog {
            record_id: Some(32),
            id: 3,
            port: "port".to_string(),
            battery_temperature: 22,
            bench_temperature_mosfet: 11,
            bench_temperature_resistor: 33,
            load: 50,
            voltage: 300,
            current: 500,
            state: "state".to_string(),
            status: "status".to_string(),
            start_date: Some("start date".to_string()),
            end_date: Some("end date".to_string()),
            test_id: 2,
        };
        thread::sleep(time::Duration::from_secs(2));
        dbg!(&log);
        on_event.send(log).unwrap();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        insert_battery_log,
        export_csv,
        parse_log,
        get_all_battery_logs,
        command_request,
        detect_serial_ports,
        populate_fake_data,
        get_all_tests,
        get_battery_logs_for_test,
        insert_test,
        delete_test,
        insert_new_test,
        data_request,
        assign_id,
        set_state
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
