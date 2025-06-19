# gRPC運用方法（Rustサーバー）

---

## 概要

RustサーバーでgRPCサービスを運用する際の設計・運用方針をまとめます。  
主に[tonic](https://docs.rs/tonic/)クレートを利用します。

---

## インストール方法

`Cargo.toml`に以下を追加し、必要なクレートをインストールします。

```toml
[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1", features = ["full"] }
```

---

## protoファイルの管理・ビルド

- `proto/`ディレクトリにgRPCサービス定義（.protoファイル）を配置
- `tonic-build`でRustコードを自動生成

### 生成コマンド例

```sh
cargo build
# またはビルド前に明示的に
cargo run --bin build_proto
```

`build.rs`やビルドスクリプトで`tonic-build`を利用して自動生成するのが一般的です。

---

## protoファイルの共通管理

- **サーバー・クライアント間で同じprotoファイルを共通利用**することで、API仕様の一貫性・型安全性を担保できます。
- プロジェクトルート等に`proto/`ディレクトリを作成し、両者から参照できるようにします。
- サーバー側・クライアント側ともに`tonic-build`で**同じprotoから型やコードを自動生成**します。
- protoファイル自体は**gitで厳密にバージョン管理**し、生成物はgit管理対象外とする運用が推奨されます。

---

## サーバー実装例

```rust
// filepath: src/login/handler.rs
use tonic::{Request, Response, Status};
use crate::login::proto::auth::{AuthServer, LoginRequest, LoginResponse};

#[derive(Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl AuthServer for AuthService {
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        // ...認証ロジック...
        let reply = LoginResponse {
            token: "dummy_token".to_string(),
        };
        Ok(Response::new(reply))
    }
}
```

---

## サーバー起動例

```rust
// filepath: src/main.rs
use tonic::transport::Server;
use login::proto::auth::auth_server::AuthServer;
use login::handler::AuthService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
```

---

## 運用上の注意

- protoファイルはサーバー・クライアント間でバージョン管理を徹底
- 生成コードはgit管理対象外（.gitignore推奨）または生成タイミングを明確化
- セキュリティ（TLS/認証）設定を適切に行う

---

## 参考

- [tonic公式ドキュメント](https://docs.rs/tonic/)
- [gRPC公式](https://grpc.io/docs/)

---
