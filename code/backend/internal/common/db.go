package common

import (
	"database/sql"
	"sync"

	_ "github.com/mattn/go-sqlite3"
)

var (
	db   *sql.DB
	once sync.Once
)

// DBインスタンスをシングルトンで取得
func GetDB(dataSourceName string) (*sql.DB, error) {
	var err error
	once.Do(func() {
		db, err = sql.Open("sqlite3", dataSourceName)
	})
	return db, err
}

// DBコネクションをクローズ
func CloseDB() error {
	if db != nil {
		return db.Close()
	}
	return nil
}
