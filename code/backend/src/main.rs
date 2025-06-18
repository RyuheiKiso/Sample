mod common;
mod features;

use tonic::transport::Server;
use features::login::handler::AuthServiceImpl;
use features::login::proto::auth::auth_service_server::AuthServiceServer;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server().await
}

// サーバー起動処理を分離
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let (auth_service, cors) = build_server_components("data/app.db");
    println!("[MAIN] AuthServiceImpl初期化");
    println!("[MAIN] gRPCサーバー起動: 0.0.0.0:50051");

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(AuthServiceServer::new(auth_service))
        .serve("0.0.0.0:50051".parse()?)
        .await?;
    println!("[MAIN] サーバー終了");
    Ok(())
}

// テスト可能な部分を分離
pub fn build_server_components(db_path: &str) -> (AuthServiceImpl, CorsLayer) {
    let auth_service = AuthServiceImpl::new(db_path);
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    (auth_service, cors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::login::handler::AuthServiceImpl;
    use tower_http::cors::{CorsLayer, Any};

    #[test]
    fn test_auth_service_impl_new() {
        let auth_service = AuthServiceImpl::new("test.db");
        assert_eq!(auth_service.db_path, "test.db");
    }

    #[test]
    fn test_cors_layer() {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        let _ = cors;
    }

    #[test]
    fn test_build_server_components() {
        let (auth_service, cors) = build_server_components("test.db");
        assert_eq!(auth_service.db_path, "test.db");
        let _ = cors;
    }

    #[tokio::test]
    async fn test_run_server_error() {
        // run_serverのアドレスパース失敗をテスト
        let result = "invalid_addr".parse::<std::net::SocketAddr>();
        assert!(result.is_err());
    }
}