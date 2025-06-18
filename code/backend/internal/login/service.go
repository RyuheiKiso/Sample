package login

import (
	"errors"
	"log"
)

type AuthService interface {
	Authenticate(username, password string) (string, error)
}

type authService struct {
	repo UserRepository
}

func NewAuthService(repo UserRepository) AuthService {
	return &authService{repo: repo}
}

func (s *authService) Authenticate(username, password string) (string, error) {
	// サービス層での認証処理開始ログ
	log.Printf("[Service] Authenticate called. Username: %s", username)
	user, err := s.repo.FindByUsername(username)
	if err != nil {
		log.Printf("[Service] ユーザー検索エラー: %v", err)
		return "", err
	}
	if user == nil || user.Password != password {
		log.Printf("[Service] 認証失敗: %s", username)
		return "", errors.New("invalid credentials")
	}
	log.Printf("[Service] 認証成功: %s", username)
	// 本来はここでJWTなどを返すが、サンプルなので固定トークン
	return "dummy-token", nil
}
