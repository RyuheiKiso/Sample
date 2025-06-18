package router

import (
	"net/http"

	"backend/internal/login"
)

func RegisterRestRoutes() {
	http.HandleFunc("/api/login", login.LoginHandler)
	// TODO: 他のRESTエンドポイントもここで登録
}
