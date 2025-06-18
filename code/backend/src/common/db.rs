use rusqlite::{Connection, Result, OpenFlags, ToSql, Row};

pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI,
    )
}

/// SELECT: 複数行取得（イテレータではなくVecで返す）
pub fn select<T, F>(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
    mut mapper: F,
) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> rusqlite::Result<T>,
{
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params)?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(mapper(row)?);
    }
    Ok(results)
}

/// INSERT/UPDATE/DELETE: 変更系クエリ実行（変更行数返却）
pub fn execute(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
) -> Result<usize> {
    conn.execute(query, params)
}