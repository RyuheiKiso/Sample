# ベースイメージとして公式のRustイメージを使用
FROM rust:latest AS builder

# Protocol Buffersコンパイラをインストール
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを作成
WORKDIR /app

# 依存ファイルをコピーして依存関係をビルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# ソースコードをコピー
COPY . .

# 本番用バイナリをビルド
RUN cargo build --release

# 実行用の軽量イメージを用意
FROM debian:bookworm-slim

WORKDIR /app

# 必要なライブラリをインストールし、パッケージを最新化（libssl-devは不要なら除外）
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/backend .

# DBファイルを含める
COPY src/data/app.db ./data/app.db

# DBファイルのパスを環境変数で指定
ENV DATABASE_URL=/app/data/app.db
EXPOSE 50051

# バイナリを実行
CMD ["/app/backend"]
