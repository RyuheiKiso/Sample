package login

import (
	"backend/internal/common"
	auth "backend/internal/login/proto/api/proto"
	"context"
)

type GrpcAuthServiceServer struct {
	auth.UnimplementedAuthServiceServer
	service AuthService
}

// 依存注入用コンストラクタ
func NewGrpcAuthServiceServer(service AuthService) *GrpcAuthServiceServer {
	return &GrpcAuthServiceServer{service: service}
}

func (s *GrpcAuthServiceServer) Login(ctx context.Context, req *auth.LoginRequest) (*auth.LoginResponse, error) {
	// contextにタイムアウトを設定する例（必要に応じて）
	// ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
	// defer cancel()

	token, err := s.service.Authenticate(ctx, req.GetUsername(), req.GetPassword())
	if err != nil {
		common.Warn("認証失敗: %s", req.GetUsername(), "gRPC")
		return nil, err
	}
	// ユーザー情報取得
	var user *User
	if authSvc, ok := s.service.(*authService); ok {
		user, _ = authSvc.repo.FindByUsername(ctx, req.GetUsername())
	}
	return &auth.LoginResponse{
		Token: token,
		User: &auth.User{
			Id:          int64(user.ID),
			Username:    user.Username,
			DisplayName: "", // DBにdisplay_nameがあれば取得してセット
		},
	}, nil
}
