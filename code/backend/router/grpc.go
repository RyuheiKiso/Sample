package router

import (
	"backend/internal/common"
	"backend/internal/login"
	auth "backend/internal/login/proto/api/proto"
	"log"

	"google.golang.org/grpc"
)

// InitAndRegisterGrpcServices: gRPCサーバーにサービスを登録する共通関数例
func InitAndRegisterGrpcServices(grpcServer *grpc.Server) {
	// DBパスは環境変数や設定ファイルから取得するのが理想
	db, err := common.GetDB("../../data/app.db")
	if err != nil {
		log.Fatalf("[gRPC] DB接続エラー: %v", err)
	}
	repo := login.NewUserRepository(db)
	service := login.NewAuthService(repo)
	authServer := login.NewGrpcAuthServiceServer(service)
	auth.RegisterAuthServiceServer(grpcServer, authServer)
}
