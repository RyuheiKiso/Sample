# Rustの環境構築

---

## 概要

Rustの開発環境を構築する手順を記載します。  
本手順ではRust公式のパッケージマネージャ「cargo」を採用します。

---

## 1. Rustのインストール

### 1.1 Rustupのインストール

Rust公式推奨のインストーラ「rustup」を使用します。  
rustupをインストールすると、Rust本体（rustc）とcargoも同時にインストールされます。

#### Windowsの場合

1. [公式サイト](https://www.rust-lang.org/ja/tools/install)からインストーラをダウンロード
2. インストーラを実行し、画面の指示に従う

または、コマンドプロンプトで以下を実行：

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### macOS/Linuxの場合

ターミナルで以下を実行：

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 2. インストール確認

インストール後、以下のコマンドでcargoとrustcのバージョンを確認します。

```sh
cargo --version
rustc --version
```

---

## 3. パスの設定

インストーラが自動でパスを設定しますが、うまくいかない場合は以下を確認してください。

- Windows: 環境変数`PATH`に`C:\Users\<ユーザー名>\.cargo\bin`が含まれていること
- macOS/Linux: `~/.cargo/bin`が`PATH`に含まれていること

---

## 4. サンプルプロジェクトの作成（cargoの利用）

cargoを使って新規プロジェクトを作成し、ビルド・実行します。

```sh
cargo new hello_rust
cd hello_rust
cargo run
```

---

## 5. 補足

- VSCode拡張機能「rust-analyzer」のインストール推奨
- 詳細は[公式ドキュメント](https://doc.rust-jp.rs/book-ja/)参照

---