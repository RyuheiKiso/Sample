# Sample

## プロジェクト概要

本プロジェクトは、Rust製バックエンドとReact（TypeScript）製フロントエンドから構成されるWebアプリケーションのサンプルです。
データベースにはSQLiteを使用し、API通信にはgRPCを利用しています。

## 技術スタック

- バックエンド: Rust, gRPC, SQLite, Docker
- フロントエンド: React, TypeScript, Jest, ESLint
- インフラ: Nginx (リバースプロキシ/SSL終端)

## 推奨環境

- OS: Windows 10/11, macOS, Linux
- Rust: 1.70以上
- Node.js: 18.x以上
- npm: 9.x以上
- Docker: 24.x以上（任意）

## ディレクトリ構成

- `code/backend/` … Rust製バックエンド
  - `src/` … ソースコード
  - `bin/migration.rs` … マイグレーション用バイナリ
  - `data/app.db` … SQLite DBファイル
  - `log/` … バックエンドのログファイル
  - `scripts/migrate.sh` … マイグレーション用シェルスクリプト
  - `tests/` … 統合テスト
- `code/frontend/` … Reactフロントエンド
  - `src/` … ソースコード
  - `public/` … 静的ファイル
  - `build/` … ビルド成果物
  - `coverage/` … テストカバレッジ
- `docs/` … ドキュメント
- `tool/nginx/` … Nginx関連ファイル

## セットアップ手順

### 1. バックエンド

1. Rustのインストール  
   [公式サイト](https://www.rust-lang.org/ja/tools/install)を参照してください。

2. 依存パッケージのインストール  
   ```sh
   cd code/backend
   cargo build
   ```

3. マイグレーションの適用  
   - Windowsの場合: `apply_migrations.bat` を実行
   - Linux/macOSの場合: `scripts/migrate.sh` を実行
   - 例:
     ```sh
     cd code/backend
     ./scripts/migrate.sh
     ```

4. サーバ起動  
   ```sh
   cargo run
   ```

5. ログファイル: `log/` 配下に出力されます。

6. DBファイル: `data/app.db` に作成されます。

7. テスト実行  
   ```sh
   cargo test
   ```

### 2. フロントエンド

1. Node.jsのインストール  
   [公式サイト](https://nodejs.org/ja/)を参照してください。

2. 依存パッケージのインストール  
   ```sh
   cd code/frontend
   npm install
   ```

3. 開発サーバ起動  
   ```sh
   npm start
   ```

4. ビルド  
   ```sh
   npm run build
   ```

5. テスト実行  
   ```sh
   npm test
   ```

6. Lint  
   ```sh
   npm run lint
   ```

### 3. その他

- 詳細な手順や運用方法は `docs/` 配下の各ドキュメントを参照してください。
- Nginxの設定例やSSL証明書は `tool/nginx/` を参照してください。

## コントリビュート

1. Issueを作成し、内容を相談してください。
2. Forkしてブランチを作成し、プルリクエストを送ってください。
3. コーディング規約・テストルールは `docs/` 配下を参照してください。

## 参考リンク

- [Rust公式](https://www.rust-lang.org/ja/)
- [React公式](https://ja.react.dev/)
- [SQLite公式](https://www.sqlite.org/index.html)
- [gRPC公式](https://grpc.io/)

## ディレクトリ構成

- `code/backend/` … Rust製バックエンド
- `code/frontend/` … Reactフロントエンド
- `docs/` … ドキュメント
- `tool/nginx/` … Nginx関連ファイル

## セットアップ手順

### 1. バックエンド

1. Rustのインストール  
   [公式サイト](https://www.rust-lang.org/ja/tools/install)を参照してください。

2. 依存パッケージのインストール  
   ```
   cd code/backend
   cargo build
   ```

3. マイグレーションの適用  
   ```
   cd code/backend
   # 必要に応じて apply_migrations.bat を実行
   ```

4. サーバ起動  
   ```
   cargo run
   ```

### 2. フロントエンド

1. Node.jsのインストール  
   [公式サイト](https://nodejs.org/ja/)を参照してください。

2. 依存パッケージのインストール  
   ```
   cd code/frontend
   npm install
   ```

3. 開発サーバ起動  
   ```
   npm start
   ```

### 3. その他

- 詳細な手順や運用方法は `docs/` 配下の各ドキュメントを参照してください。

## ライセンス

本リポジトリはサンプル用途です。ライセンスは適宜設定してください。