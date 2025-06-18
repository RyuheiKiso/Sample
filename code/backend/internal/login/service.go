package login

import (
	"backend/internal/common"
	"context"
	"errors"
)

// AuthServiceは認証サービスのインターフェース
type AuthService interface {
	common.Service
	Authenticate(ctx context.Context, username, password string) (string, error)
}

type authService struct {
	common.BaseService // 共通フィールド・メソッド用
	repo               UserRepository
}

func NewAuthService(repo UserRepository) AuthService {
	return &authService{
		BaseService: common.BaseService{},
		repo:        repo,
	}
}

func (s *authService) Authenticate(ctx context.Context, username, password string) (string, error) {
	// サービス層での認証処理開始ログ
	common.Info("Authenticate called. Username: %s", username, "Service")
	user, err := s.repo.FindByUsername(ctx, username)
	if err != nil {
		common.Error("ユーザー検索エラー: %v", err, "Service")
		return "", err
	}
	if user == nil || user.Password != password {
		common.Warn("認証失敗: %s", username, "Service")
		return "", errors.New("invalid credentials")
	}
	common.Info("認証成功: %s", username, "Service")
	// 本来はここでJWTなどを返すが、サンプルなので固定トークン
	return "dummy-token", nil
}

// HealthCheckの実装（common.Serviceインターフェース）
func (s *authService) HealthCheck(ctx context.Context) error {
	// 必要に応じてDB疎通などを実装
	return s.BaseService.HealthCheck(ctx)
}
