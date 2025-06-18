mod common;
mod features;

use tonic::transport::Server;
use features::login::handler::AuthServiceImpl;
use features::login::proto::auth::auth_service_server::AuthServiceServer;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[MAIN] AuthServiceImpl初期化");
    let auth_service = AuthServiceImpl::new("data/app.db");

    println!("[MAIN] gRPCサーバー起動: 0.0.0.0:50051");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

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
        .serve("0.0.0.0:50051".parse()?)
        .await?;
    println!("[MAIN] サーバー終了");
    Ok(())
}
