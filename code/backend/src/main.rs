use axum::{Router, routing::any};
use tower_http::cors::{CorsLayer, Any};
use tower::Layer;

use tonic_web::GrpcWebLayer;
use sqlx::sqlite::SqlitePool;
use dotenvy;
mod core;
mod feature;
use crate::core::generated::auth::auth_service_server::AuthServiceServer;
use crate::feature::login::login::grpc_handler::GrpcLoginHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = "0.0.0.0:50051";
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://src/data/app.db".to_string());
    let db = SqlitePool::connect(&db_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secretkey".to_string());
    let auth_service = GrpcLoginHandler {
        pool: db.clone(),
        jwt_secret: jwt_secret.clone(),
    };
    println!("gRPC server listening on {} (with gRPC-Web)", addr);
    let grpc = GrpcWebLayer::new().layer(AuthServiceServer::new(auth_service));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(grpc)
        .layer(cors);

    use axum::serve;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await?;
    Ok(())
}
