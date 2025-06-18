# テーブル設計書

---

## 名称

ユーザー

---

## 定義

| フィールド名    | 型           | 主キー | NOT NULL | ユニーク | デフォルト値 | 説明           |
|-----------------|--------------|--------|----------|----------|--------------|----------------|
| id              | INTEGER      | ○      | ○        | ○        | AUTOINCREMENT| ユーザーID     |
| username        | TEXT         |        | ○        | ○        |              | ユーザー名     |
| password        | TEXT         |        | ○        |          |              | パスワード（平文可） |
| display_name    | TEXT         |        | ○        |          |              | 表示名         |
| created_at      | DATETIME     |        | ○        |          | CURRENT_TIMESTAMP | 作成日時   |
| updated_at      | DATETIME     |        | ○        |          | CURRENT_TIMESTAMP | 更新日時   |

---

## サンプル

| id | username | password | display_name | created_at          | updated_at          |
|----|----------|----------|--------------|---------------------|---------------------|
| 1  | alice    | alicepw  | Alice        | 2024-06-01 10:00:00 | 2024-06-01 10:00:00 |
| 2  | bob      | bobpw    | Bob          | 2024-06-01 10:05:00 | 2024-06-01 10:05:00 |

---