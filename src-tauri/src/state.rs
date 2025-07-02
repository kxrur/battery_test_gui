use diesel::SqliteConnection;

#[derive(Default)]
pub struct AppState {
    pub db_path: String,
    pub db_connection: Option<SqliteConnection>,
}
