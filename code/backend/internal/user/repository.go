package user

import (
	"backend/internal/common"
	"context"
)

// UserRepositoryは共通Repositoryインターフェースを拡張
type UserRepository interface {
	common.Repository
}

type userRepository struct{}

func NewUserRepository() UserRepository {
	return &userRepository{}
}

// --- 共通Repositoryインターフェースのダミー実装 ---
func (r *userRepository) GetByID(ctx context.Context, id int) (interface{}, error) {
	return nil, nil
}
func (r *userRepository) Create(ctx context.Context, entity interface{}) error {
	return nil
}
func (r *userRepository) Update(ctx context.Context, entity interface{}) error {
	return nil
}
func (r *userRepository) Delete(ctx context.Context, id int) error {
	return nil
}
