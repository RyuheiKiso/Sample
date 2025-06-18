use crate::features::login::repository::{UserRepository, UserRecord};
use anyhow::{Result, anyhow};

pub struct LoginService<R: UserRepository> {
    pub repo: R,
}

impl<R: UserRepository> LoginService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<UserRecord> {
        println!("[SERVICE] authenticate: username={}", username);
        let user = self.repo.find_by_username(username)?;
        if let Some(user) = user {
            println!("[SERVICE] ユーザー発見: username={}", user.username);
            if user.password == password {
                println!("[SERVICE] パスワード一致: username={}", user.username);
                Ok(user)
            } else {
                println!("[SERVICE] パスワード不一致: username={}", user.username);
                Err(anyhow!("Invalid password"))
            }
        } else {
            println!("[SERVICE] ユーザー未発見: username={}", username);
            Err(anyhow!("User not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockRepo;
    impl UserRepository for MockRepo {
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
    fn test_authenticate_success() {
        let service = LoginService::new(MockRepo);
        let user = service.authenticate("alice", "alicepw").unwrap();
        assert_eq!(user.username, "alice");
    }

    #[test]
    fn test_authenticate_wrong_password() {
        let service = LoginService::new(MockRepo);
        assert!(service.authenticate("alice", "wrongpw").is_err());
    }

    #[test]
    fn test_authenticate_user_not_found() {
        let service = LoginService::new(MockRepo);
        assert!(service.authenticate("bob", "bobpw").is_err());
    }
}
