package login

import (
	"backend/internal/common"
	"encoding/json"
	"log"
	"net/http"
	"os"
)

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginResponse struct {
	Token string `json:"token,omitempty"`
	Error string `json:"error,omitempty"`
}

func LoginHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		log.Printf("[REST] Invalid method: %s", r.Method)
		w.WriteHeader(http.StatusMethodNotAllowed)
		json.NewEncoder(w).Encode(LoginResponse{Error: "Method not allowed"})
		return
	}

	var req LoginRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		log.Printf("[REST] JSON decode error: %v", err)
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(LoginResponse{Error: "Invalid request"})
		return
	}

	// DBパスは環境変数や設定ファイルから取得するのが理想だが、ここでは直書き
	dbPath := "../data/app.db"
	if envPath := os.Getenv("DB_PATH"); envPath != "" {
		dbPath = envPath
	}
	db, err := common.GetDB(dbPath)
	if err != nil {
		log.Printf("[REST] DB接続エラー: %v", err)
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(LoginResponse{Error: "DB接続エラー"})
		return
	}
	repo := NewUserRepository(db)
	service := NewAuthService(repo)
	token, err := service.Authenticate(req.Username, req.Password)
	if err != nil {
		log.Printf("[REST] 認証失敗: %s", req.Username)
		w.WriteHeader(http.StatusUnauthorized)
		json.NewEncoder(w).Encode(LoginResponse{Error: "Invalid credentials"})
		return
	}
	log.Printf("[REST] 認証成功: %s", req.Username)
	json.NewEncoder(w).Encode(LoginResponse{Token: token})
}
