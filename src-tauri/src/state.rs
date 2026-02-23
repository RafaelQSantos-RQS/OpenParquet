use duckdb::Connection;
use parking_lot::Mutex;

pub struct AppState {
    pub conn: Mutex<Connection>,
}

impl AppState {
    pub fn new() -> Result<Self, duckdb::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
