package login

import (
	"backend/internal/common"
	auth "backend/internal/login/proto/api/proto"
	"context"
	"log"
	"os"
)

type GrpcAuthServiceServer struct {
	auth.UnimplementedAuthServiceServer
}

func NewGrpcAuthServiceServer() *GrpcAuthServiceServer {
	return &GrpcAuthServiceServer{}
}

func (s *GrpcAuthServiceServer) Login(ctx context.Context, req *auth.LoginRequest) (*auth.LoginResponse, error) {
	dbPath := "../../data/app.db"
	if envPath := os.Getenv("DB_PATH"); envPath != "" {
		dbPath = envPath
	}
	db, err := common.GetDB(dbPath)
	if err != nil {
		log.Printf("[gRPC] DB接続エラー: %v", err)
		return nil, err
	}
	repo := NewUserRepository(db)
	service := NewAuthService(repo)
	token, err := service.Authenticate(req.GetUsername(), req.GetPassword())
	if err != nil {
		log.Printf("[gRPC] 認証失敗: %s", req.GetUsername())
		return nil, err
	}
	log.Printf("[gRPC] 認証成功: %s", req.GetUsername())
	user, _ := repo.FindByUsername(req.GetUsername())
	return &auth.LoginResponse{
		Token: token,
		User: &auth.User{
			Id:          int64(user.ID),
			Username:    user.Username,
			DisplayName: "", // DBにdisplay_nameがあれば取得してセット
		},
	}, nil
}
