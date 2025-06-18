package common

import (
	"context"
	"database/sql"
)

// DBRepositoryはDBを使った共通CRUD実装
// Tはエンティティ型、ScanFuncはDB行→エンティティ変換関数

type ScanFunc[T any] func(row *sql.Row) (T, error)

type DBRepository[T any] struct {
	DB      *sql.DB
	Table   string
	ScanRow ScanFunc[T]
}

func (r *DBRepository[T]) GetByID(ctx context.Context, id int) (interface{}, error) {
	row := r.DB.QueryRowContext(ctx, "SELECT * FROM "+r.Table+" WHERE id = ?", id)
	return r.ScanRow(row)
}

// Create/Update/DeleteはエンティティごとにSQLが異なるため、必要に応じてラップして使うことを推奨
// デフォルト実装はエラーを返すのみ
func (r *DBRepository[T]) Create(ctx context.Context, entity interface{}) error {
	return sql.ErrConnDone // 未実装
}
func (r *DBRepository[T]) Update(ctx context.Context, entity interface{}) error {
	return sql.ErrConnDone // 未実装
}
func (r *DBRepository[T]) Delete(ctx context.Context, id int) error {
	return sql.ErrConnDone // 未実装
}
