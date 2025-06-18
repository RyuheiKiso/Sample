use tonic::{Request, Response, Status};
use crate::features::login::proto::auth::auth_service_server::AuthService;
use crate::features::login::proto::auth::{LoginRequest, LoginResponse, User};
use crate::features::login::service::LoginService;
use crate::features::login::repository::SqliteUserRepository;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct AuthServiceImpl {
    pub service: LoginService<SqliteUserRepository>,
}

impl AuthServiceImpl {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        let repo = SqliteUserRepository { conn };
        let service = LoginService::new(repo);
        Self { service }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let user = self.service.authenticate(&req.username, &req.password)
            .map_err(|_| Status::unauthenticated("Invalid username or password"))?;
        let resp = LoginResponse {
            token: "dummy_token".to_string(),
            user: Some(User {
                id: user.id,
                username: user.username,
                display_name: user.display_name,
            }),
        };
        Ok(Response::new(resp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::login::repository::UserRecord;

    struct MockRepo;
    impl UserRepository for MockRepo {
        fn find_by_username(&self, username: &str) -> anyhow::Result<Option<UserRecord>> {
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

    #[tokio::test]
    async fn test_login_success() {
        let service = LoginService::new(MockRepo);
        let handler = AuthServiceImpl { service };
        let req = Request::new(LoginRequest {
            username: "alice".to_string(),
            password: "alicepw".to_string(),
        });
        let resp = handler.login(req).await.unwrap().into_inner();
        assert_eq!(resp.user.unwrap().username, "alice");
    }

    #[tokio::test]
    async fn test_login_fail() {
        let service = LoginService::new(MockRepo);
        let handler = AuthServiceImpl { service };
        let req = Request::new(LoginRequest {
            username: "alice".to_string(),
            password: "wrongpw".to_string(),
        });
        let result = handler.login(req).await;
        assert!(result.is_err());
    }
}
