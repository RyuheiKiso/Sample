package login

import (
	"context"
	"errors"
	"log"
	"os"
)

var serviceLogger = log.New(os.Stdout, "[Service] ", log.LstdFlags)

type AuthService interface {
	Authenticate(ctx context.Context, username, password string) (string, error)
}

type authService struct {
	repo UserRepository
}

func NewAuthService(repo UserRepository) AuthService {
	return &authService{repo: repo}
}

func (s *authService) Authenticate(ctx context.Context, username, password string) (string, error) {
	// サービス層での認証処理開始ログ
	serviceLogger.Printf("Authenticate called. Username: %s", username)
	user, err := s.repo.FindByUsername(ctx, username)
	if err != nil {
		serviceLogger.Printf("ユーザー検索エラー: %v", err)
		return "", err
	}
	if user == nil || user.Password != password {
		serviceLogger.Printf("認証失敗: %s", username)
		return "", errors.New("invalid credentials")
	}
	serviceLogger.Printf("認証成功: %s", username)
	// 本来はここでJWTなどを返すが、サンプルなので固定トークン
	return "dummy-token", nil
}
