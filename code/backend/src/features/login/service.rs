use crate::features::login::repository::{UserRepository, UserRecord};
use anyhow::{Result, anyhow};

// ログクロージャを持てるように
pub struct LoginService<R: UserRepository> {
    pub repo: R,
    pub logger: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

impl<R: UserRepository> LoginService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo, logger: None }
    }
    pub fn with_logger<F>(repo: R, logger: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        Self { repo, logger: Some(Box::new(logger)) }
    }
    fn log(&self, msg: &str) {
        match &self.logger {
            Some(logger) => logger(msg),
            None => println!("{}", msg),
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<UserRecord> {
        self.log(&format!("[SERVICE] authenticate: username={}", username));
        let user = self.repo.find_by_username(username)?;
        if let Some(user) = user {
            self.log(&format!("[SERVICE] ユーザー発見: username={}", user.username));
            if user.password == password {
                self.log(&format!("[SERVICE] パスワード一致: username={}", user.username));
                Ok(user)
            } else {
                self.log(&format!("[SERVICE] パスワード不一致: username={}", user.username));
                Err(anyhow!("Invalid password"))
            }
        } else {
            self.log(&format!("[SERVICE] ユーザー未発見: username={}", username));
            Err(anyhow!("User not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::login::repository::DummyRepo;
    use std::sync::{Arc, Mutex};
    use anyhow::anyhow;

    #[test]
    fn test_authenticate_cases() {
        let service = LoginService::with_logger(DummyRepo, |_msg| {});
        let cases = vec![
            ("alice", "alicepw", true, None),
            ("alice", "wrongpw", false, Some("Invalid password")),
            ("bob", "bobpw", false, Some("User not found")),
        ];
        for (username, password, should_succeed, expected_err) in cases {
            let result = service.authenticate(username, password);
            assert_eq!(result.is_ok(), should_succeed, "case: {} {}", username, password);
            if let (Err(e), Some(msg)) = (result, expected_err) {
                assert!(e.to_string().contains(msg));
            }
        }
    }

    #[test]
    fn test_logger_none_branch() {
        // logger: None の場合もカバレッジ
        let service = LoginService::new(DummyRepo);
        // 標準出力に出るが、パニックしないことを確認
        let _ = service.authenticate("alice", "alicepw");
    }

    #[test]
    fn test_logger_called() {
        // loggerが呼ばれることを確認
        let logs = Arc::new(Mutex::new(Vec::new()));
        let logs_clone = logs.clone();
        let service = LoginService::with_logger(DummyRepo, move |msg| {
            logs_clone.lock().unwrap().push(msg.to_string());
        });
        let _ = service.authenticate("alice", "alicepw");
        assert!(logs.lock().unwrap().iter().any(|l| l.contains("authenticate")));
    }

    struct ErrorRepo;
    impl UserRepository for ErrorRepo {
        fn find_by_username(&self, _username: &str) -> anyhow::Result<Option<UserRecord>> {
            Err(anyhow!("db error"))
        }
    }

    #[test]
    fn test_authenticate_repo_error() {
        let service = LoginService::with_logger(ErrorRepo, |_msg| {});
        let result = service.authenticate("alice", "alicepw");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("db error"));
    }
}