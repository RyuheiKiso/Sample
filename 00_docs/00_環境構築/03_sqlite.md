# sqliteの環境構築

---

## 概要

SQLiteは軽量な組み込み型データベースです。ここではWindows環境でのSQLiteの導入手順を説明します。

---

## インストール手順

### 1. SQLite公式サイトからダウンロード

- [SQLite Download Page](https://www.sqlite.org/download.html) にアクセスします。
- 「Precompiled Binaries for Windows」セクションから以下の2つのzipファイルをダウンロードします。
  - sqlite-tools-win32-x86-xxxxxxx.zip（コマンドラインツール一式）
  - sqlite-dll-win64-x64-xxxxxxx.zip（64bit DLL）

### 2. ファイルの展開

- ダウンロードしたzipファイルを任意のフォルダに展開します（例: `C:\sqlite`）。

### 3. パスの設定（任意）

- コマンドプロンプトからどこでもsqlite3コマンドを使いたい場合は、展開したフォルダを「システム環境変数PATH」に追加します。

### 4. 動作確認

- コマンドプロンプトを開き、以下のコマンドを実行します。

  ```
  sqlite3 --version
  ```

- バージョン情報が表示されればインストール完了です。

---

## サンプル：データベース作成とテーブル作成

1. データベースファイル作成

   ```
   sqlite3 sample.db
   ```

2. テーブル作成

   ```
   CREATE TABLE users (
     id INTEGER PRIMARY KEY,
     name TEXT NOT NULL
   );
   ```

3. データ挿入

   ```
   INSERT INTO users (name) VALUES ('山田太郎');
   ```

4. データ確認

   ```
   SELECT * FROM users;
   ```

---

## 参考リンク

- [SQLite公式ドキュメント](https://www.sqlite.org/docs.html)
- [SQLite日本語リファレンス](https://www.dbonline.jp/sqlite/)

---
