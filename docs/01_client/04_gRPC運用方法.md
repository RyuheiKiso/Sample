# gRPC運用方法（Reactクライアント）

---

## 概要

ReactクライアントでgRPC通信を行う際の運用方法・設計方針についてまとめます。  
主にgRPC-Webを利用し、TypeScript型安全な開発を推奨します。

---

## インストール方法

gRPC-Webクライアントや型生成ツールをインストールします。

```sh
npm install @protobuf-ts/grpcweb-transport
npm install --save-dev @protobuf-ts/plugin protoc
# または
npm install grpc-web
npm install --save-dev ts-proto protoc
```

- 利用する型生成ツール（protobuf-ts, ts-proto等）に応じて選択してください。

---

## 利用ライブラリ例

- [grpc-web](https://github.com/grpc/grpc-web)  
  gRPCをWebブラウザから利用するための公式クライアント
- [protobuf-ts](https://github.com/timostamm/protobuf-ts) / [ts-proto](https://github.com/stephenh/ts-proto)  
  TypeScript用のprotoファイル→型生成ツール

---

## protoファイルの管理・型生成

- protoファイルは`features/<機能>/proto/`ディレクトリに配置
- `protobuf-ts`や`ts-proto`等でTypeScript型を自動生成
- 生成物は`proto/`配下にコミットし、型安全なgRPC通信を実現

### 型生成コマンド例（protobuf-ts）

```sh
npx protoc --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
  --ts_out=src/features/login/proto \
  --proto_path=proto proto/auth.proto
```

---

## gRPCクライアントの実装例

```typescript
// filepath: src/features/login/api/loginApi.ts
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { AuthServiceClient } from "../proto/auth.client";
import { LoginRequest } from "../proto/auth";

const transport = new GrpcWebFetchTransport({
  baseUrl: "https://api.example.com", // gRPCサーバーのエンドポイント
});

const client = new AuthServiceClient(transport);

export async function login(username: string, password: string) {
  const req: LoginRequest = { username, password };
  const { response } = await client.login(req);
  return response;
}
```

---

## CORS・プロキシ設定

- gRPC-WebはCORS対応が必要
- 開発時は`vite.config.ts`や`setupProxy.js`でプロキシ設定を行い、  
  gRPCサーバーへのリクエストを中継

---

## テスト

- gRPCクライアントのテストはAPI層でモック化して実施
- proto型のモックデータを作成し、ユニットテストで利用

---

## 運用上の注意

- protoファイルのバージョン管理を徹底
- サーバー・クライアントで型生成タイミングを揃える
- 生成物の差分レビューを行う

---

## 参考

- [gRPC-Web公式ドキュメント](https://grpc.io/docs/platforms/web/)
- [protobuf-tsドキュメント](https://github.com/timostamm/protobuf-ts)
- [ts-protoドキュメント](https://github.com/stephenh/ts-proto)

---
