use tonic::transport::Channel;
use crate::features::login::proto::auth::auth_service_client::AuthServiceClient;
use crate::features::login::proto::auth::LoginRequest;

pub async fn login_with_grpc(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[CLIENT] gRPCクライアント接続開始: http://[::1]:50051");
    let mut client = AuthServiceClient::connect("http://[::1]:50051").await?;
    println!("[CLIENT] gRPCクライアント接続成功");

    println!("[CLIENT] リクエスト送信: username={}, password={}", username, password);
    let request = tonic::Request::new(LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    });

    let response = client.login(request).await?;
    println!("[CLIENT] レスポンス受信: {:?}", response.into_inner());
    Ok(())
}
