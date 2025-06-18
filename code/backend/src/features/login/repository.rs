use anyhow::Result;
use crate::common::db::{get_connection, select, select_with_hook};

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub display_name: String,
}

pub trait UserRepository: Send + Sync + 'static {
    fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>>;
}

pub struct SqliteUserRepository {
    pub db_path: String,
    // ログ用クロージャを追加（デフォルトはprintln!）
    pub logger: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

impl SqliteUserRepository {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
            logger: None,
        }
    }
    pub fn with_logger<F>(db_path: &str, logger: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        Self {
            db_path: db_path.to_string(),
            logger: Some(Box::new(logger)),
        }
    }
    fn log(&self, msg: &str) {
        match &self.logger {
            Some(logger) => logger(msg),
            None => println!("{}", msg),
        }
    }
}

impl Clone for SqliteUserRepository {
    fn clone(&self) -> Self {
        Self {
            db_path: self.db_path.clone(),
            logger: None,
        }
    }
}

impl UserRepository for SqliteUserRepository {
    fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>> {
        self.log(&format!("[REPO] find_by_username: username={}", username));
        let conn = get_connection(&self.db_path)?;
        let query = "SELECT id, username, password, display_name FROM user WHERE username = ?1";
        // select_with_hookでログ出力
        let results = select_with_hook(
            &conn,
            query,
            &[&username],
            |row| Ok(UserRecord {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                display_name: row.get(3)?,
            }),
            |msg| self.log(msg),
        )?;
        if let Some(user) = results.get(0) {
            self.log(&format!("[REPO] ユーザー取得成功: username={}", username));
            Ok(Some(user.clone()))
        } else {
            self.log(&format!("[REPO] ユーザー取得失敗: username={}", username));
            Ok(None)
        }
    }
}

// DummyRepoをpubにして再利用しやすく
pub struct DummyRepo;
impl UserRepository for DummyRepo {
    fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>> {
        if username == "alice" {
            Ok(Some(UserRecord {
                id: 1,
                username: "alice".to_string(),
                password: "alicepw".to_string(),
                display_name: "Alice".to_string(),
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db::get_connection;
    use std::fs;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_find_by_username_cases() {
        let repo = DummyRepo;
        let cases = vec![
            ("alice", Some("alice")),
            ("bob", None),
        ];
        for (input, expected) in cases {
            let result = repo.find_by_username(input).unwrap();
            match (result, expected) {
                (Some(user), Some(name)) => assert_eq!(user.username, name),
                (None, None) => (),
                _ => panic!("unexpected result"),
            }
        }
    }

    #[test]
    fn test_sqlite_user_repository_found_and_not_found() {
        // 一時ファイルDBを使う
        let db_path = "test_user_repo.db";
        {
            let conn = get_connection(db_path).unwrap();
            conn.execute(
                "CREATE TABLE user (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    display_name TEXT NOT NULL
                )",
                [],
            ).unwrap();
            conn.execute(
                "INSERT INTO user (username, password, display_name) VALUES (?1, ?2, ?3)",
                ["alice", "alicepw", "Alice"],
            ).unwrap();
        }

        // ログを抑制するクロージャを渡す
        let repo = SqliteUserRepository::with_logger(db_path, |_msg| {});
        let user = repo.find_by_username("alice").unwrap().unwrap();
        assert_eq!(user.username, "alice");
        assert!(repo.find_by_username("bob").unwrap().is_none());

        // テスト後にファイル削除
        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn test_logger_none_branch() {
        // logger: None の場合もカバレッジ
        let db_path = "test_user_repo2.db";
        {
            let conn = get_connection(db_path).unwrap();
            conn.execute(
                "CREATE TABLE user (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    display_name TEXT NOT NULL
                )",
                [],
            ).unwrap();
            conn.execute(
                "INSERT INTO user (username, password, display_name) VALUES (?1, ?2, ?3)",
                ["alice", "alicepw", "Alice"],
            ).unwrap();
        }
        let repo = SqliteUserRepository::new(db_path);
        let _ = repo.find_by_username("alice");
        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn test_logger_called() {
        let db_path = "test_user_repo3.db";
        {
            let conn = get_connection(db_path).unwrap();
            conn.execute(
                "CREATE TABLE user (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    display_name TEXT NOT NULL
                )",
                [],
            ).unwrap();
            conn.execute(
                "INSERT INTO user (username, password, display_name) VALUES (?1, ?2, ?3)",
                ["alice", "alicepw", "Alice"],
            ).unwrap();
        }
        let logs = Arc::new(Mutex::new(Vec::new()));
        let logs_clone = logs.clone();
        let repo = SqliteUserRepository::with_logger(db_path, move |msg| {
            logs_clone.lock().unwrap().push(msg.to_string());
        });
        let _ = repo.find_by_username("alice");
        assert!(logs.lock().unwrap().iter().any(|l| l.contains("find_by_username")));
        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_clone_logger_none() {
        let repo = SqliteUserRepository::with_logger("dummy", |_msg| {});
        let cloned = repo.clone();
        assert_eq!(cloned.db_path, repo.db_path);
        assert!(cloned.logger.is_none());
    }

    #[test]
    fn test_find_by_username_db_error() {
        // 存在しないDBファイルで接続エラーを発生させる
        let repo = SqliteUserRepository::with_logger("///invalid_path/xxx.db", |_msg| {});
        let result = repo.find_by_username("alice");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_by_username_sql_error() {
        // テーブルが存在しない場合のSQLエラー
        let db_path = "test_user_repo_no_table.db";
        {
            let _conn = get_connection(db_path).unwrap();
            // テーブルを作らない
        }
        let repo = SqliteUserRepository::with_logger(db_path, |_msg| {});
        let result = repo.find_by_username("alice");
        assert!(result.is_err());
        let _ = fs::remove_file(db_path);
    }
}