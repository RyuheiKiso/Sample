
use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tower::Layer;
use log::info;
use tonic_web::GrpcWebLayer;
use sqlx::sqlite::SqlitePool;
use dotenvy;
// TLS関連のuseを削除
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tower::Service;
mod core;
mod feature;
mod common;
use crate::core::generated::auth::auth_service_server::AuthServiceServer;
use crate::feature::login::grpc_handler::GrpcLoginHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    common::logger::init_logger();
    let addr = "0.0.0.0:50051";
    // TLSを使わないため証明書・鍵のパス不要
    // [注意] 本番環境ではTLS終端をリバースプロキシ等で必ず行い、通信の暗号化を徹底してください。
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::warn!("DATABASE_URLが設定されていません。デフォルトを使用します");
        "sqlite://src/data/app.db".to_string()
    });
    let db = SqlitePool::connect(&db_url).await.map_err(|e| {
        log::error!("DB接続失敗: {}", e);
        e
    })?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        log::warn!("JWT_SECRETが設定されていません。デフォルトを使用します");
        "secretkey".to_string()
    });
    let auth_service = GrpcLoginHandler {
        pool: db.clone(),
        jwt_secret: jwt_secret.clone(),
    };
    info!("gRPC server listening on {} (with gRPC-Web, no TLS)", addr);
    let grpc = GrpcWebLayer::new().layer(AuthServiceServer::new(auth_service));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(grpc)
        .layer(cors);

    // 通常のTCPで受けてそのままserve_connection
    let tcp_listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        log::error!("サーバーリッスン失敗: {}", e);
        e
    })?;
    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let app = app.clone();
        tokio::spawn(async move {
            let service = app.clone();
            let svc = service;
            let io = TokioIo::new(stream);
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| svc.clone().call(req)))
                .await
            {
                log::error!("接続の処理中にエラー: {}", e);
            }
        });
    }
    // never reached
    // Ok(())
}
