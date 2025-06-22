# React環境構築

---

## 概要

Reactアプリケーションの開発環境を構築する手順を記載します。

---

## 前提条件

- Node.js（推奨: 最新のLTSバージョン）がインストールされていること
- npm

---

## 1. Node.jsのインストール

公式サイトからインストーラーをダウンロードしてインストールします。

- [Node.js公式サイト](https://nodejs.org/ja/)

インストール後、以下のコマンドでバージョンを確認します。

```sh
node -v
npm -v
```

---

## 2. プロジェクトの作成

`create-react-app`を使ってReactプロジェクトを作成します。

```sh
npx create-react-app my-app
cd my-app
```

---

## 3. 開発サーバーの起動

```sh
npm start
```

ブラウザで `http://localhost:3000` を開くとReactアプリが表示されます。

---

## 4. 主なコマンド

- 開発サーバー起動: `npm start`
- ビルド: `npm run build`
- テスト: `npm test`
- ライブラリ追加: `npm install パッケージ名`

---

## 5. 補足

- TypeScriptで作成する場合は以下のコマンドを使用します。

```sh
npx create-react-app my-app --template typescript
```



## 6. Protocol Buffers Compiler (protoc) のインストール（macOS）

### Homebrewを使う場合

```sh
brew install protobuf
```

インストール後、バージョン確認：

```sh
protoc --version
```

### npm scriptsで利用する場合

`package.json` の `devDependencies` に `protoc` が含まれているため、下記コマンドでインストールできます。

```sh
npm install
```

> **注意:**
> npm でインストールできる `protoc` パッケージは公式の Protocol Buffers Compiler とは異なります。
> gRPC や protobuf ファイルのコンパイル等、公式の `protoc` コマンドが必要な場合は Homebrew などでインストールしてください。

