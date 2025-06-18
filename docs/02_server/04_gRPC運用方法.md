# gRPC運用方法（Goサーバー）

---

## 概要

GoサーバーでgRPCサービスを運用する際の設計・運用方針をまとめます。  
主に[google.golang.org/grpc](https://pkg.go.dev/google.golang.org/grpc)パッケージを利用します。

---

## インストール方法

`go.mod`に以下を追加し、必要なパッケージをインストールします。

```sh
go get google.golang.org/grpc
go get google.golang.org/protobuf/cmd/protoc-gen-go
go get google.golang.org/grpc/cmd/protoc-gen-go-grpc
```

---

## protoファイルの管理・ビルド

- `proto/`ディレクトリにgRPCサービス定義（.protoファイル）を配置
- `protoc`コマンドでGoコードを自動生成

### 生成コマンド例

```sh
protoc --go_out=internal/login/proto --go-grpc_out=internal/login/proto -I proto proto/auth.proto
```

---

## protoファイルの共通管理

- **サーバー・クライアント間で同じprotoファイルを共通利用**することで、API仕様の一貫性・型安全性を担保できます。
- プロジェクトルート等に`proto/`ディレクトリを作成し、両者から参照できるようにします。
- サーバー側・クライアント側ともに`protoc`で**同じprotoから型やコードを自動生成**します。
- protoファイル自体は**gitで厳密にバージョン管理**し、生成物はgit管理対象外とする運用が推奨されます。

---

## サーバー実装例

```go
// filepath: internal/login/handler.go
package login

import (
    context "context"
    pb "backend/internal/login/proto"
)

type AuthService struct {
    pb.UnimplementedAuthServer
}

func (s *AuthService) Login(ctx context.Context, req *pb.LoginRequest) (*pb.LoginResponse, error) {
    // ...認証ロジック...
    return &pb.LoginResponse{
        Token: "dummy_token",
    }, nil
}
```

---

## サーバー起動例

```go
// filepath: cmd/server/main.go
package main

import (
    "log"
    "net"
    "google.golang.org/grpc"
    "backend/internal/login"
    pb "backend/internal/login/proto"
)

func main() {
    lis, err := net.Listen("tcp", ":50051")
    if err != nil {
        log.Fatalf("failed to listen: %v", err)
    }
    s := grpc.NewServer()
    pb.RegisterAuthServer(s, &login.AuthService{})
    if err := s.Serve(lis); err != nil {
        log.Fatalf("failed to serve: %v", err)
    }
}
```

---

## 運用上の注意

- protoファイルはサーバー・クライアント間でバージョン管理を徹底
- 生成コードはgit管理対象外（.gitignore推奨）または生成タイミングを明確化
- セキュリティ（TLS/認証）設定を適切に行う

---

## 参考

- [gRPC-Go公式ドキュメント](https://grpc.io/docs/languages/go/)
- [gRPC公式](https://grpc.io/docs/)

---
- 生成コードはgit管理対象外（.gitignore推奨）または生成タイミングを明確化
- セキュリティ（TLS/認証）設定を適切に行う

---

## 参考

- [tonic公式ドキュメント](https://docs.rs/tonic/)
- [gRPC公式](https://grpc.io/docs/)

---
