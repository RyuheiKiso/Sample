
use tonic::transport::Server;
use sqlx::sqlite::SqlitePool;
use dotenvy;
mod core;
mod feature;
use crate::core::generated::generated::auth::auth_service_server::AuthServiceServer;
use crate::feature::login::login::grpc_handler::GrpcLoginHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = "0.0.0.0:50051".parse()?;
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://src/data/app.db".to_string());
    let db = SqlitePool::connect(&db_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secretkey".to_string());
    let auth_service = GrpcLoginHandler {
        pool: db,
        jwt_secret,
    };
    println!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;
    Ok(())
}
