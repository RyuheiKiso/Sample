
use tonic::{Request, Response, Status};
use crate::core::generated::generated::auth::{LoginRequest, LoginResponse, User as ProtoUser};
use crate::core::generated::generated::auth::auth_service_server::AuthService;
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
        let req = request.into_inner();
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
                Ok(Response::new(resp))
            },
            Err(e) => Err(Status::unauthenticated(format!("認証失敗: {}", e)))
        }
    }
}
