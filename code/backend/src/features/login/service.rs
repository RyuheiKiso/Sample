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
        let user = self.repo.find_by_username(username)?;
        if let Some(user) = user {
            if user.password == password {
                Ok(user)
            } else {
                Err(anyhow!("Invalid password"))
            }
        } else {
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
