use diesel::PgConnection;
use std::sync::Mutex;

pub struct AppState {
    /// The connection to the database.
    pub db_conn: Mutex<PgConnection>,
}

impl AppState {
    /// Creates a new app state
    pub fn new(db_conn: PgConnection) -> Self {
        Self {
            db_conn: Mutex::new(db_conn),
        }
    }
}
