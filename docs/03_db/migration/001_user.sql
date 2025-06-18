DROP TABLE IF EXISTS user;

CREATE TABLE IF NOT EXISTS user (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  display_name TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- サンプルデータ挿入
INSERT INTO user (username, password, display_name, created_at, updated_at) VALUES
  ('alice', 'alicepw', 'Alice', '2024-06-01 10:00:00', '2024-06-01 10:00:00'),
  ('bob', 'bobpw', 'Bob', '2024-06-01 10:05:00', '2024-06-01 10:05:00');
