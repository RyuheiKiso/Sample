use sqlx::Row;


use tonic::{transport::Server, Request, Response, Status};
mod generated;
use generated::auth::{auth_service_server::{AuthService, AuthServiceServer}, LoginRequest, LoginResponse, User};
use sqlx::sqlite::SqlitePool;
use jwt::SignWithKey;
use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;




#[derive(Debug, Clone)]
pub struct MyAuthService {
    db: SqlitePool,
    jwt_secret: String,
}


#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: i64,
    exp: usize,
}

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        // ユーザー検索
        let row = sqlx::query(
            r#"SELECT id, username, password, display_name FROM user WHERE username = ?"#
        )
        .bind(&req.username)
        .fetch_optional(&self.db)
        .await
        .map_err(|_| Status::internal("DB error"))?;

        let user_row = match row {
            Some(u) => u,
            None => return Err(Status::unauthenticated("ユーザー名またはパスワードが不正です")),
        };
        // パスワード平文比較（本番はハッシュ化推奨）
        let db_password: String = user_row.try_get("password").unwrap();
        if db_password != req.password {
            return Err(Status::unauthenticated("ユーザー名またはパスワードが不正です"));
        }

        // JWT生成
        let exp = chrono::Utc::now().timestamp() as usize + 60 * 60; // 1時間有効
        let claims = Claims {
            sub: user_row.try_get::<String, _>("username").unwrap(),
            user_id: user_row.try_get::<i64, _>("id").unwrap(),
            exp,
        };
        let key = HmacSha256::new_from_slice(self.jwt_secret.as_bytes()).unwrap();
        let token = claims.sign_with_key(&key).map_err(|_| Status::internal("JWT生成失敗"))?;

        let user = User {
            id: user_row.try_get::<i64, _>("id").unwrap(),
            username: user_row.try_get::<String, _>("username").unwrap(),
            display_name: user_row.try_get::<String, _>("display_name").unwrap(),
        };
        let reply = LoginResponse {
            token,
            user: Some(user),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = "0.0.0.0:50051".parse()?;
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://src/data/app.db".to_string());
    let db = SqlitePool::connect(&db_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secretkey".to_string());
    let auth_service = MyAuthService {
        db,
        jwt_secret,
    };
    println!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
