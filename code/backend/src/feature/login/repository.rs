
use sqlx::SqlitePool;
use anyhow::Result;
use log::{debug};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub display_name: String,
}

pub struct UserRepository<'a> {
    pub pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        debug!("[UserRepository] find_by_username: '{}'", username);
        let rec = sqlx::query_as::<_, User>(
            r#"SELECT id, username, password, display_name FROM user WHERE username = ?"#
        )
        .bind(username)
        .fetch_optional(self.pool)
        .await?;
        if rec.is_some() {
            debug!("[UserRepository] ユーザー '{}' 見つかり", username);
        } else {
            debug!("[UserRepository] ユーザー '{}' 見つからず", username);
        }
        Ok(rec)
    }
}
