use diesel::SqliteConnection;

use crate::database::models::Test;

#[derive(Default)]
pub struct AppState {
    pub db_path: String,
    pub db_connection: Option<SqliteConnection>,
    pub tests: Vec<Test>,
}
