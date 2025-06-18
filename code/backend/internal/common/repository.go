package common

import "context"

// Repositoryは基本的なCRUD操作の共通インターフェース例
// Tはエンティティ型、Kは主キー型
// 実際の実装ではジェネリクスが使えないため、各リポジトリで具体的に実装してください

type Repository interface {
	GetByID(ctx context.Context, id int) (interface{}, error)
	Create(ctx context.Context, entity interface{}) error
	Update(ctx context.Context, entity interface{}) error
	Delete(ctx context.Context, id int) error
}
