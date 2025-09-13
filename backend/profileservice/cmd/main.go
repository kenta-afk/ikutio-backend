package main

import (
	"os"
	"log/slog"
	"database/sql"  
    _ "github.com/go-sql-driver/mysql"
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
	}  
	defer db.Close()
	
}