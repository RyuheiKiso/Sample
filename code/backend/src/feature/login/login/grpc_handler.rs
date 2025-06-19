use log::{info, warn, debug};

use tonic::{Request, Response, Status};
use crate::core::generated::auth::{LoginRequest, LoginResponse, User as ProtoUser};
use crate::core::generated::auth::auth_service_server::AuthService;
use crate::feature::login::login::service::LoginService;
use sqlx::SqlitePool;

pub struct GrpcLoginHandler {
    pub pool: SqlitePool,
    pub jwt_secret: String,
}

#[tonic::async_trait]
impl AuthService for GrpcLoginHandler {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("gRPC login: リクエスト受信");
        let req = request.into_inner();
        debug!("gRPC login: username='{}'", req.username);
        let user_repo = crate::feature::login::login::repository::UserRepository { pool: &self.pool };
        let service = LoginService { user_repo, jwt_secret: &self.jwt_secret };
        match service.login(&req.username, &req.password).await {
            Ok((token, user)) => {
                let proto_user = ProtoUser {
                    id: user.id,
                    username: user.username,
                    display_name: user.display_name,
                };
                let resp = LoginResponse {
                    token,
                    user: Some(proto_user),
                };
                info!("gRPC login: ユーザー '{}' ログイン成功", req.username);
                Ok(Response::new(resp))
            },
            Err(e) => {
                warn!("gRPC login: ユーザー '{}' ログイン失敗: {}", req.username, e);
                Err(Status::unauthenticated(format!("認証失敗: {}", e)))
            }
        }
    }
}
