use anyhow::Result;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::common::db::{get_connection, find_user_by_username};

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
        // クエリをfeatures側で指定
        let query = "SELECT id, username, password, display_name FROM user WHERE username = ?1";
        if let Some((id, username, password, display_name)) = find_user_by_username(&conn, query, username)? {
            println!("[REPO] ユーザー取得成功: username={}", username);
            Ok(Some(UserRecord {
                id,
                username,
                password,
                display_name,
            }))
        } else {
            println!("[REPO] ユーザー取得失敗: username={}", username);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
    }
}
