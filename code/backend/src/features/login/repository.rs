use anyhow::Result;
use crate::common::db::{get_connection, select};

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

#[derive(Clone)]
pub struct SqliteUserRepository {
    pub db_path: String,
}

impl UserRepository for SqliteUserRepository {
    fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>> {
        println!("[REPO] find_by_username: username={}", username);
        let conn = get_connection(&self.db_path)?;
        let query = "SELECT id, username, password, display_name FROM user WHERE username = ?1";
        let results = select(
            &conn,
            query,
            &[&username],
            |row| Ok(UserRecord {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                display_name: row.get(3)?,
            }),
        )?;
        if let Some(user) = results.get(0) {
            println!("[REPO] ユーザー取得成功: username={}", username);
            Ok(Some(user.clone()))
        } else {
            println!("[REPO] ユーザー取得失敗: username={}", username);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db::get_connection;
    use std::fs;

    struct DummyRepo;
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

    #[test]
    fn test_find_by_username_found() {
        let repo = DummyRepo;
        let user = repo.find_by_username("alice").unwrap().unwrap();
        assert_eq!(user.username, "alice");
    }

    #[test]
    fn test_find_by_username_not_found() {
        let repo = DummyRepo;
        assert!(repo.find_by_username("bob").unwrap().is_none());
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

        let repo = SqliteUserRepository { db_path: db_path.to_string() };
        let user = repo.find_by_username("alice").unwrap().unwrap();
        assert_eq!(user.username, "alice");
        assert!(repo.find_by_username("bob").unwrap().is_none());

        // テスト後にファイル削除
        let _ = fs::remove_file(db_path);
    }
}