# Rustの環境構築

---

## 概要

Rustの開発環境を構築する手順を記載します。  
本手順ではRust公式のインストーラ「rustup」を利用します。

---

## 1. Rustのインストール

### 1.1 rustupによるインストール

Rust公式サイトからインストーラをダウンロードしてインストールします。

#### Windowsの場合

1. [Rust公式サイト](https://www.rust-lang.org/ja/tools/install)からインストーラ（rustup-init.exe）をダウンロード
2. インストーラを実行し、画面の指示に従う

#### macOS/Linuxの場合

ターミナルで以下を実行：

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

※詳細は公式サイトで最新手順を確認してください。

---

## 2. インストール確認

インストール後、以下のコマンドでRustのバージョンを確認します。

```sh
rustc --version
cargo --version
```

---

## 3. パスの設定

Rustのバイナリパスが通っていることを確認してください。

- Windows: 環境変数`PATH`に`C:\Users\<ユーザー名>\.cargo\bin`が含まれていること
- macOS/Linux: `~/.cargo/bin`が`PATH`に含まれていること

---

## 4. サンプルプロジェクトの作成

Rustで新規プロジェクトを作成し、ビルド・実行します。

```sh
cargo new hello_rust
cd hello_rust
cargo run
```

---

## 5. 補足

- VSCode拡張機能「rust-analyzer」のインストール推奨
- 詳細は[公式ドキュメント](https://doc.rust-lang.org/book/)参照

---