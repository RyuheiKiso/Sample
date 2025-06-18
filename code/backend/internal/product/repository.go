package product

import (
	"backend/internal/common"
	"context"
)

// ProductRepositoryは共通Repositoryインターフェースを拡張
type ProductRepository interface {
	common.Repository
}

type productRepository struct{}

func NewProductRepository() ProductRepository {
	return &productRepository{}
}

// --- 共通Repositoryインターフェースのダミー実装 ---
func (r *productRepository) GetByID(ctx context.Context, id int) (interface{}, error) {
	return nil, nil
}
func (r *productRepository) Create(ctx context.Context, entity interface{}) error {
	return nil
}
func (r *productRepository) Update(ctx context.Context, entity interface{}) error {
	return nil
}
func (r *productRepository) Delete(ctx context.Context, id int) error {
	return nil
}
