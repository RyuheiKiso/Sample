
use crate::feature::login::repository::{UserRepository, User};
use log::{info, warn, debug};
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
        info!("login: ユーザー名='{}' でログイン試行", username);
        let user = self.user_repo.find_by_username(username).await?;
        if user.is_none() {
            warn!("login: ユーザー '{}' が存在しません", username);
            return Err(anyhow!("ユーザーが存在しません"));
        }
        let user = user.unwrap();
        // パスワード検証（平文）
        if user.password != password {
            warn!("login: ユーザー '{}' のパスワード不一致", username);
            return Err(anyhow!("パスワードが一致しません"));
        }
        info!("login: ユーザー '{}' の認証成功", username);
        // JWT生成
        let mut claims = BTreeMap::new();
        claims.insert("sub", user.username.clone());
        claims.insert("exp", (chrono::Utc::now().timestamp() + 3600).to_string());
        let key: Hmac<Sha256> = <Hmac<Sha256> as sha2::digest::KeyInit>::new_from_slice(self.jwt_secret.as_bytes()).unwrap();
        let token_str = claims.sign_with_key(&key)?;
        debug!("login: JWT生成完了 for '{}', token(一部)='{}...'", username, &token_str[..8]);
        Ok((token_str, user))
    }
}
