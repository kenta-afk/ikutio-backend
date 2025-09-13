FROM golang:1.25.1-alpine3.22
  
WORKDIR /app  
  
# 必要なツールをインストール  
RUN apk add --no-cache git ca-certificates tzdata  
  
# Goの最適化設定  
ENV CGO_ENABLED=0  
ENV GOOS=linux  
ENV GOARCH=arm64  
  
# ビルドキャッシュとモジュールキャッシュ用のディレクトリを作成  
RUN mkdir -p /go/pkg/mod /root/.cache/go-build 
  
# go.modとgo.sumを先にコピー（依存関係キャッシュ最適化）  
COPY go.mod go.sum ./  
RUN go mod download  
  
# ソースコードをコピー  
COPY . .  
  
EXPOSE 50051
  
# 開発時は直接実行
CMD ["go", "run", "cmd/main.go"]
