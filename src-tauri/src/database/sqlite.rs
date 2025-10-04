use std::fs;
use std::sync::Mutex;
use thiserror::Error;

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::{Manager, State};

use crate::database::models::{BatteryLog, Test};
use crate::state::AppState;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to get app data directory: {0}")]
    AppDataDir(#[source] tauri::Error),
    #[error("Failed to create app data directory: {0}")]
    CreateDir(#[source] std::io::Error),
    #[error("Failed to convert path to string")]
    PathConversion,
    #[error("Database connection error: {0}")]
    Connection(#[source] diesel::result::ConnectionError),
    #[error("Migration error: {0}")]
    Migration(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Database operation error: {0}")]
    Operation(#[source] diesel::result::Error),
}

#[tauri::command]
#[specta::specta]
pub fn get_all_battery_logs(statee: State<'_, Mutex<AppState>>) -> Result<Vec<BatteryLog>, String> {
    let mut app_state = statee.lock().map_err(|e| e.to_string())?;

    if app_state.db_connection.is_none() {
        app_state.db_connection =
            Some(establish_connection(&app_state.db_path).map_err(|e| e.to_string())?);
    }

    let connection = app_state.db_connection.as_mut().unwrap();
    use crate::database::schema::battery_logs::dsl::*;
    battery_logs
        .load::<BatteryLog>(connection)
        .map_err(|e| format!("Failed to load battery logs: {}", e))
}

#[tauri::command]
#[specta::specta]
pub fn insert_battery_log(
    state: State<'_, Mutex<AppState>>,
    log_data: BatteryLog,
) -> Result<BatteryLog, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    if state.db_connection.is_none() {
        state.db_connection =
            Some(establish_connection(&state.db_path).map_err(|e| e.to_string())?);
    }

    let connection = state.db_connection.as_mut().unwrap();

    diesel::insert_into(crate::database::schema::battery_logs::table)
        .values(&log_data)
        .execute(connection)
        .map_err(|e| e.to_string())?;

    let inserted_log = crate::database::schema::battery_logs::table
        .order(crate::database::schema::battery_logs::record_id.desc())
        .first(connection)
        .map_err(|e| e.to_string())?;

    Ok(inserted_log)
}
#[tauri::command]
#[specta::specta]
pub fn insert_test(statee: State<'_, Mutex<AppState>>, test: Test) -> Result<Test, String> {
    let mut statee = statee.lock().map_err(|e| e.to_string())?;

    if statee.db_connection.is_none() {
        statee.db_connection =
            Some(establish_connection(&statee.db_path).map_err(|e| e.to_string())?);
    }

    let conn = statee.db_connection.as_mut().unwrap();

    diesel::insert_into(crate::database::schema::tests::table)
        .values(&test)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    let inserted = crate::database::schema::tests::table
        .order(crate::database::schema::tests::test_id.desc())
        .first(conn)
        .map_err(|e| e.to_string())?;

    Ok(inserted)
}

#[tauri::command]
#[specta::specta]
pub fn get_battery_logs_for_test(
    statee: State<'_, Mutex<AppState>>,
    target_test_id: i32,
) -> Result<Vec<BatteryLog>, String> {
    let mut statee = statee.lock().map_err(|e| e.to_string())?;

    if statee.db_connection.is_none() {
        statee.db_connection =
            Some(establish_connection(&statee.db_path).map_err(|e| e.to_string())?);
    }

    let conn = statee.db_connection.as_mut().unwrap();
    use crate::database::schema::battery_logs::dsl::*;

    battery_logs
        .filter(test_id.eq(target_test_id))
        .load::<BatteryLog>(conn)
        .map_err(|e| format!("Failed to get logs for test {}: {}", target_test_id, e))
}

#[tauri::command]
#[specta::specta]
pub fn get_all_tests(state: State<'_, Mutex<AppState>>) -> Result<Vec<Test>, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    if state.db_connection.is_none() {
        state.db_connection =
            Some(establish_connection(&state.db_path).map_err(|e| e.to_string())?);
    }

    let conn = state.db_connection.as_mut().unwrap();

    use crate::database::schema::tests::dsl::*;
    tests
        .load::<Test>(conn)
        .map_err(|e| format!("Failed to load tests: {}", e))
}

pub fn init_database(app_handle: &tauri::AppHandle) -> Result<(), DatabaseError> {
    // Get app data directory
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(DatabaseError::AppDataDir)?;

    // Create directory if it doesn't exist
    fs::create_dir_all(&app_dir).map_err(DatabaseError::CreateDir)?;

    // Set database path
    let db_path = app_handle
        .path()
        .app_data_dir()
        .map_err(DatabaseError::AppDataDir)?
        .join("battery_logs.db");
    let db_path_str = db_path.to_str().ok_or(DatabaseError::PathConversion)?;
    dbg!(&db_path_str);
    let state = app_handle.state::<Mutex<AppState>>();

    let mut state = state.lock().unwrap();
    state.db_path = db_path_str.to_string();

    let mut connection = establish_connection(&db_path_str)?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(DatabaseError::Migration)?;

    state.db_connection = Some(connection);

    Ok(())
}

pub fn establish_connection(db_path_str: &str) -> Result<SqliteConnection, DatabaseError> {
    SqliteConnection::establish(db_path_str).map_err(DatabaseError::Connection)
}
