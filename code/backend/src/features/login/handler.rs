use tonic::{Request, Response, Status};
use crate::features::login::proto::auth::auth_service_server::AuthService;
use crate::features::login::proto::auth::{LoginRequest, LoginResponse, User};
use crate::features::login::service::LoginService;
use crate::features::login::repository::SqliteUserRepository;

pub struct AuthServiceImpl {
    pub db_path: String,
}

impl AuthServiceImpl {
    pub fn new(db_path: &str) -> Self {
        Self { db_path: db_path.to_string() }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        println!("[LOGIN] リクエスト受信: username={}, password={}", req.username, req.password);

        // 必要なときにDB接続
        // let conn = match get_connection(&self.db_path) {
        //     Ok(conn) => conn,
        //     Err(e) => {
        //         println!("[LOGIN] DB接続失敗: {}", e);
        //         return Err(Status::internal("DB connection failed"));
        //     }
        // };
        // db_pathを渡す形に修正
        let repo = SqliteUserRepository { db_path: self.db_path.clone() };
        let service = LoginService::new(repo);

        let user = match service.authenticate(&req.username, &req.password) {
            Ok(user) => {
                println!("[LOGIN] 認証成功: username={}", user.username);
                user
            }
            Err(e) => {
                println!("[LOGIN] 認証失敗: username={}, error={}", req.username, e);
                return Err(Status::unauthenticated("Invalid username or password"));
            }
        };

        let resp = LoginResponse {
            token: "dummy_token".to_string(),
            user: Some(User {
                id: user.id,
                username: user.username,
                display_name: user.display_name,
            }),
        };
        println!("[LOGIN] レスポンス送信: token={}, user={:?}", resp.token, resp.user);

        Ok(Response::new(resp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::login::proto::auth::{LoginRequest};
    use crate::features::login::service::LoginService;
    use crate::features::login::repository::{UserRecord, UserRepository};
    use crate::common::db::select;
    use tonic::{Request, Response, Status};
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    // テスト用リポジトリ（コネクションを直接持つ）
    #[derive(Clone)]
    struct TestUserRepository {
        conn: Arc<Mutex<Connection>>,
    }
    impl UserRepository for TestUserRepository {
        fn find_by_username(&self, username: &str) -> anyhow::Result<Option<UserRecord>> {
            let conn = self.conn.lock().unwrap();
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
                Ok(Some(user.clone()))
            } else {
                Ok(None)
            }
        }
    }

    // テスト用ハンドラ
    struct TestAuthServiceImpl {
        service: LoginService<TestUserRepository>,
    }
    #[tonic::async_trait]
    impl AuthService for TestAuthServiceImpl {
        async fn login(
            &self,
            request: Request<LoginRequest>,
        ) -> Result<Response<LoginResponse>, Status> {
            let req = request.into_inner();
            let user = match self.service.authenticate(&req.username, &req.password) {
                Ok(user) => user,
                Err(_) => return Err(Status::unauthenticated("Invalid username or password")),
            };
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

    fn setup_test_repo() -> TestUserRepository {
        let conn = Connection::open_in_memory().unwrap();
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
        TestUserRepository { conn: Arc::new(Mutex::new(conn)) }
    }

    #[tokio::test]
    async fn test_login_success() {
        let repo = setup_test_repo();
        let service = LoginService::new(repo);
        let handler = TestAuthServiceImpl { service };
        let req = Request::new(LoginRequest {
            username: "alice".to_string(),
            password: "alicepw".to_string(),
        });
        let resp = handler.login(req).await.unwrap().into_inner();
        assert_eq!(resp.user.unwrap().username, "alice");
    }

    #[tokio::test]
    async fn test_login_fail() {
        let repo = setup_test_repo();
        let service = LoginService::new(repo);
        let handler = TestAuthServiceImpl { service };
        let req = Request::new(LoginRequest {
            username: "alice".to_string(),
            password: "wrongpw".to_string(),
        });
        let result = handler.login(req).await;
        assert!(result.is_err());
    }
}