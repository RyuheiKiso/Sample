package login

import (
	auth "backend/internal/login/proto/api/proto"
	"context"
	"log"
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

	token, err := s.service.Authenticate(req.GetUsername(), req.GetPassword())
	if err != nil {
		log.Printf("[gRPC] 認証失敗: %s", req.GetUsername())
		return nil, err
	}
	// ユーザー情報取得
	user, _ := s.service.(*authService).repo.FindByUsername(req.GetUsername())
	return &auth.LoginResponse{
		Token: token,
		User: &auth.User{
			Id:          int64(user.ID),
			Username:    user.Username,
			DisplayName: "", // DBにdisplay_nameがあれば取得してセット
		},
	}, nil
}
