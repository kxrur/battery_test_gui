use specta_typescript::Typescript;
use std::sync::Mutex;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

mod database;

mod state;

use database::export::export_csv;
use database::sqlite::init_database;

use crate::{database::sqlite::insert_battery_log, state::AppState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder =
        Builder::<tauri::Wry>::new().commands(collect_commands![insert_battery_log, export_csv]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
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
