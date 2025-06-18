use rusqlite::{Connection, Result};

pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
}
