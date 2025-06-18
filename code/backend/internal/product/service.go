package product

import (
	"backend/internal/common"
	"context"
)

// ProductServiceは商品機能のサービスインターフェース
type ProductService interface {
	common.Service
	// ここに商品固有のメソッドを追加
}

type productService struct {
	common.BaseService
}

func NewProductService() ProductService {
	return &productService{
		BaseService: common.BaseService{},
	}
}

// HealthCheckの実装（common.Serviceインターフェース）
func (s *productService) HealthCheck(ctx context.Context) error {
	return s.BaseService.HealthCheck(ctx)
}
