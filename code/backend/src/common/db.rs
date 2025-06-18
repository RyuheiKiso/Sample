use rusqlite::{Connection, Result};

pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
}

// クエリを引数で受け取るように変更
pub fn find_user_by_username(
    conn: &Connection,
    query: &str,
    username: &str,
) -> Result<Option<(i64, String, String, String)>> {
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([username])?;
    if let Some(row) = rows.next()? {
        Ok(Some((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        )))
    } else {
        Ok(None)
    }
}
