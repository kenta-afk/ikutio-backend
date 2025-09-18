package main

import (
	"database/sql"
	"fmt"
	"log/slog"
	"net"
	"os"

	_ "github.com/go-sql-driver/mysql"
	"google.golang.org/grpc"

	pb "github.com/kenta-afk/ikutio-backend/build"
	"github.com/kenta-afk/ikutio-backend/internal/application"
	"github.com/kenta-afk/ikutio-backend/internal/infrastructure"
)

func main() {
	log_level := os.Getenv("LOG_LEVEL")
	programLevel := new(slog.LevelVar)

	switch log_level {
	case "DEBUG":
		programLevel.Set(slog.LevelDebug)
	case "WARN":
		programLevel.Set(slog.LevelWarn)
	case "ERROR":
		programLevel.Set(slog.LevelError)
	default:
		programLevel.Set(slog.LevelInfo)
	}
	// JSONハンドラーを作成
	h := slog.NewJSONHandler(os.Stderr, &slog.HandlerOptions{Level: programLevel})

	// デフォルトロガーに設定
	slog.SetDefault(slog.New(h))

	db_url := os.Getenv("DATABASE_URL")
	db, err := sql.Open("mysql", db_url)
	if err != nil {
		slog.Error("Failed to connect to database", "error", err)
		return
	}
	defer db.Close()

	// DBコネクションをテスト
	if err := db.Ping(); err != nil {
		slog.Error("Failed to ping database", "error", err)
		return
	}

	// gRPCサーバーの初期化
	grpcServer := grpc.NewServer()

	// Repository とProfileServiceの初期化
	profileRepo := infrastructure.New(db)
	profileService := application.New(profileRepo)
	profileGRPCService := application.NewProfileGRPCService(profileService)

	// gRPCサーバーにサービスを登録
	pb.RegisterProfileServiceServer(grpcServer, profileGRPCService)

	// リスナーの設定
	port := os.Getenv("PORT")
	if port == "" {
		port = "50051"
	}

	listener, err := net.Listen("tcp", fmt.Sprintf(":%s", port))
	if err != nil {
		slog.Error("Failed to listen", "error", err, "port", port)
		return
	}

	slog.Info("gRPC server starting", "port", port)

	// サーバー開始
	if err := grpcServer.Serve(listener); err != nil {
		slog.Error("Failed to serve gRPC server", "error", err)
	}
}
