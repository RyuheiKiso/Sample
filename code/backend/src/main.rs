mod common;
mod features;

use tonic::transport::Server;
use features::login::handler::AuthServiceImpl;
use features::login::proto::auth::auth_service_server::AuthServiceServer;
use common::db::get_connection;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("data/app.db")?;
    let conn = Arc::new(Mutex::new(conn));
    let auth_service = AuthServiceImpl::new(conn);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve("[::1]:50051".parse()?)
        .await?;
    Ok(())
}
