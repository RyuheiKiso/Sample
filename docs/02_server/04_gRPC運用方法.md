# gRPC運用方法（Rustサーバー）

---

## 概要

RustサーバーでgRPCサービスを運用する際の設計・運用方針をまとめます。  
主に[tonic](https://github.com/hyperium/tonic)クレートを利用します。

---

## インストール方法

Cargo.tomlに以下を追加し、必要なクレートをインストールします。

```toml
[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1", features = ["full"] }
```

- コマンド例:
  ```sh
  cargo add tonic prost tokio --features tokio/full
  ```

- gRPCコード生成用に`tonic-build`も追加します（build.rsで利用）。

```toml
[build-dependencies]
tonic-build = "0.10"
```

---

## protoファイルの管理・ビルド

- `proto/`ディレクトリにgRPCサービス定義（.protoファイル）を配置
- `build.rs`で`tonic-build`を利用し、Rustコードを自動生成

### build.rs例

```rust
// filepath: backend/build.rs
fn main() {
    tonic_build::configure()
        .out_dir("src/features/login/proto") // 生成先
        .compile(&["proto/auth.proto"], &["proto"])
        .unwrap();
}
```

---

## protoファイルの共通管理

- **サーバー・クライアント間で同じprotoファイルを共通利用**することで、API仕様の一貫性・型安全性を担保できます。
- プロジェクトルート等に`proto/`ディレクトリを作成し、両者から参照できるようにします。
- サーバー側は`tonic-build`、クライアント側は`protobuf-ts`や`ts-proto`等で**同じprotoから型やコードを自動生成**します。
- protoファイル自体は**gitで厳密にバージョン管理**し、生成物はgit管理対象外とする運用が推奨されます。

---

## サーバー実装例

```rust
// filepath: src/features/login/handler.rs
use tonic::{Request, Response, Status};
use crate::features::login::proto::auth::{LoginRequest, LoginResponse};
use crate::features::login::proto::auth_server::{Auth, AuthServer};

#[derive(Default)]
pub struct AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        // ...認証ロジック...
        Ok(Response::new(LoginResponse {
            token: "dummy_token".into(),
        }))
    }
}
```

---

## サーバー起動例

```rust
// filepath: src/main.rs
use tonic::transport::Server;
use crate::features::login::handler::{AuthService, AuthServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(AuthServer::new(AuthService::default()))
        .serve("[::1]:50051".parse()?)
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
