FROM golang:1.25.1-alpine3.22

WORKDIR /app

# 依存関係管理ファイルをコピー
COPY go.mod go.sum ./

# 依存関係をダウンロード
RUN go mod download

# ソースコードをコピー
COPY . .

# アプリケーションをビルド
RUN go build -o profileservice main.go

# ポート設定
EXPOSE 50051

# アプリケーション実行
CMD ["./profileservice"]
