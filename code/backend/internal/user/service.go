package user

import (
	"backend/internal/common"
	"context"
)

// UserServiceはユーザー機能のサービスインターフェース
type UserService interface {
	common.Service
	// ここにユーザー固有のメソッドを追加
}

type userService struct {
	common.BaseService
}

func NewUserService() UserService {
	return &userService{
		BaseService: common.BaseService{},
	}
}

// HealthCheckの実装（common.Serviceインターフェース）
func (s *userService) HealthCheck(ctx context.Context) error {
	return s.BaseService.HealthCheck(ctx)
}
