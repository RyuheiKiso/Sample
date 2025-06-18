package main

import (
	"log"
	"net/http"
	"os"
	"sync"

	login "backend/internal/login"
	"backend/router"

	"github.com/improbable-eng/grpc-web/go/grpcweb"
	"google.golang.org/grpc"
)

func main() {
	var wg sync.WaitGroup

	// RESTサーバー
	log.Println("Setting up REST handler for /api/login")
	http.HandleFunc("/api/login", login.LoginHandler)
	wg.Add(1)
	go func() {
		log.Println("Starting REST server at :8080")
		if err := http.ListenAndServe(":8080", nil); err != nil {
			log.Println("REST server error:", err)
			os.Exit(1)
		}
		wg.Done()
	}()

	// gRPCサーバー
	log.Println("Setting up gRPC server")
	grpcServer := grpc.NewServer()
	router.InitAndRegisterGrpcServices(grpcServer)

	// gRPC-Webラッパー
	grpcWebServer := grpcweb.WrapServer(grpcServer,
		grpcweb.WithOriginFunc(func(origin string) bool { return true }), // CORS全許可
	)

	wg.Add(1)
	go func() {
		mux := http.NewServeMux()
		mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
			if grpcWebServer.IsGrpcWebRequest(r) || grpcWebServer.IsAcceptableGrpcCorsRequest(r) {
				grpcWebServer.ServeHTTP(w, r)
				return
			}
			w.WriteHeader(http.StatusNotFound)
		})
		log.Println("Starting gRPC-Web server at :50051")
		if err := http.ListenAndServe(":50051", mux); err != nil {
			log.Println("gRPC-Web server error:", err)
			os.Exit(1)
		}
		wg.Done()
	}()

	wg.Wait()
}
