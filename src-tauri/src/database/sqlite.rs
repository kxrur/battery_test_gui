use std::fs;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::Manager;
use thiserror::Error;

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
    Migration(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[tauri::command]
pub fn init_database(app_handle: &tauri::AppHandle) -> Result<(), DatabaseError> {
    // Get app data directory
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(DatabaseError::AppDataDir)?;
    dbg!(&app_dir);

    // Create directory if it doesn't exist
    fs::create_dir_all(&app_dir).map_err(DatabaseError::CreateDir)?;

    // Set database path
    let db_path = app_dir.join("battery_logs.db");
    let db_path_str = db_path.to_str().ok_or(DatabaseError::PathConversion)?;

    // Establish connection and run migrations
    let mut connection = establish_connection(db_path_str)?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(DatabaseError::Migration)?;

    Ok(())
}

// Helper function to establish connection in other parts of your app
pub fn establish_connection(db_path: &str) -> Result<SqliteConnection, DatabaseError> {
    SqliteConnection::establish(db_path).map_err(DatabaseError::Connection)
}
