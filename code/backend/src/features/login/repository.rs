use anyhow::Result;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

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
    pub conn: Arc<Mutex<Connection>>,
}

impl UserRepository for SqliteUserRepository {
    fn find_by_username(&self, username: &str) -> Result<Option<UserRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, password, display_name FROM user WHERE username = ?1"
        )?;
        let mut rows = stmt.query([username])?;
        if let Some(row) = rows.next()? {
            Ok(Some(UserRecord {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                display_name: row.get(3)?,
            }))
        } else {
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
