use rusqlite::{Connection, Result, OpenFlags, ToSql, Row};

pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_URI,
    )
}

fn select_internal<T, F, L>(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
    mut mapper: F,
    mut logger: Option<L>,
) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> rusqlite::Result<T>,
    L: FnMut(&str),
{
    if let Some(ref mut log) = logger {
        log(&format!("SQL: {}", query));
    }
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params)?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(mapper(row)?);
    }
    Ok(results)
}

/// SELECT: 複数行取得（イテレータではなくVecで返す）
pub fn select<T, F>(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
    mapper: F,
) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> rusqlite::Result<T>,
{
    select_internal(conn, query, params, mapper, None::<fn(&str)>)
}

/// SELECT: 複数行取得（イテレータではなくVecで返す）
/// ログやエラー処理を外部から注入できるようにするため、
/// 追加のクロージャ引数を受け取れるようにする例（既存関数はそのまま）
pub fn select_with_hook<T, F, L>(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
    mapper: F,
    logger: L,
) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> rusqlite::Result<T>,
    L: FnMut(&str),
{
    select_internal(conn, query, params, mapper, Some(logger))
}

/// INSERT/UPDATE/DELETE: 変更系クエリ実行（変更行数返却）
pub fn execute(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
) -> Result<usize> {
    conn.execute(query, params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE user (id INTEGER PRIMARY KEY, username TEXT NOT NULL)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO user (username) VALUES (?1)",
            ["alice"],
        ).unwrap();
        conn
    }

    #[test]
    fn test_select_and_select_with_hook() {
        let conn = setup_conn();
        // loggerなし
        let users: Vec<(i64, String)> = select(
            &conn,
            "SELECT id, username FROM user WHERE username = ?1",
            &[&"alice"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].1, "alice");

        // loggerあり
        let logs = Arc::new(Mutex::new(Vec::new()));
        let logs_clone = logs.clone();
        let users2: Vec<(i64, String)> = select_with_hook(
            &conn,
            "SELECT id, username FROM user WHERE username = ?1",
            &[&"alice"],
            |row| Ok((row.get(0)?, row.get(1)?)),
            move |msg| logs_clone.lock().unwrap().push(msg.to_string()),
        ).unwrap();
        assert_eq!(users2.len(), 1);
        assert!(logs.lock().unwrap().iter().any(|l| l.contains("SQL:")));
    }

    #[test]
    fn test_execute() {
        let conn = setup_conn();
        let affected = execute(
            &conn,
            "UPDATE user SET username = ?1 WHERE username = ?2",
            &[&"bob", &"alice"],
        ).unwrap();
        assert_eq!(affected, 1);
    }

    #[test]
    fn test_select_error_branch() {
        let conn = Connection::open_in_memory().unwrap();
        // テーブルがないのでエラーになる
        let result: rusqlite::Result<Vec<(i64, String)>> = select(
            &conn,
            "SELECT id, username FROM user WHERE username = ?1",
            &[&"alice"],
            |row| Ok((row.get(0)?, row.get(1)?)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_select_with_hook_error_branch() {
        let conn = Connection::open_in_memory().unwrap();
        let logs = Arc::new(Mutex::new(Vec::new()));
        let logs_clone = logs.clone();
        let result: rusqlite::Result<Vec<(i64, String)>> = select_with_hook(
            &conn,
            "SELECT id, username FROM user WHERE username = ?1",
            &[&"alice"],
            |row| Ok((row.get(0)?, row.get(1)?)),
            move |msg| logs_clone.lock().unwrap().push(msg.to_string()),
        );
        assert!(result.is_err());
        assert!(logs.lock().unwrap().iter().any(|l| l.contains("SQL:")));
    }

    #[test]
    fn test_execute_error_branch() {
        let conn = Connection::open_in_memory().unwrap();
        // テーブルがないのでエラーになる
        let result = execute(
            &conn,
            "UPDATE user SET username = ?1 WHERE username = ?2",
            &[&"bob", &"alice"],
        );
        assert!(result.is_err());
    }
}