
use crate::feature::login::login::repository::{UserRepository, User};
use anyhow::{Result, anyhow};
use jwt::SignWithKey;
use hmac::Hmac;
use sha2::Sha256;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct LoginService<'a> {
    pub user_repo: UserRepository<'a>,
    pub jwt_secret: &'a str,
}

impl<'a> LoginService<'a> {
    pub async fn login(&self, username: &str, password: &str) -> Result<(String, User)> {
        let user = self.user_repo.find_by_username(username).await?
            .ok_or_else(|| anyhow!("ユーザーが存在しません"))?;
        // パスワード検証（平文）
        if user.password != password {
            return Err(anyhow!("パスワードが一致しません"));
        }
        // JWT生成
        let mut claims = BTreeMap::new();
        claims.insert("sub", user.username.clone());
        claims.insert("exp", (chrono::Utc::now().timestamp() + 3600).to_string());
        let key: Hmac<Sha256> = <Hmac<Sha256> as sha2::digest::KeyInit>::new_from_slice(self.jwt_secret.as_bytes()).unwrap();
        let token_str = claims.sign_with_key(&key)?;
        Ok((token_str, user))
    }
}
