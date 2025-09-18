# ikutio-backend

ゲームプラットフォーム「ikutio」のバックエンドシステム。マイクロサービスアーキテクチャで構築されており、認証、プロファイル管理、ゲーム機能を提供します。

## アーキテクチャ

このプロジェクトは以下のサービスで構成されています：

### サービス構成

- **AuthService** (Rust) - ユーザー認証とトークン管理
- **ProfileService** (Go) - ユーザープロファイル管理
- **GameService** (Rust) - ゲーム機能とロジック
- **BFF (Backend for Frontend)** (Rust) - フロントエンド向けAPIゲートウェイ

### データベース

- **MySQL** - ユーザー情報、ゲームデータ
- **PostgreSQL** - 追加のデータ管理
- **DynamoDB** - NoSQL データストレージ

## 前提条件

- **Rust** 1.70以上 (Cargoツールチェーン)
- **Go** 1.19以上
- **Docker** & **Docker Compose**
- **Protocol Buffers Compiler** (protoc)

## クイックスタート

### 1. 依存関係のインストール

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Go
brew install go  # macOS
# または https://golang.org/dl/ からダウンロード

# Protocol Buffers
brew install protobuf  # macOS
```

### 2. データベースの起動

```bash
# Docker Composeでデータベースサービスを起動
docker compose up -d
```

### 3. プロトコルバッファの生成

```bash
# ProfileServiceのプロトファイル生成
cd backend/profileservice
make all

# その他のプロトファイルはCargoビルドで自動生成されます
```

### 4. サービスの起動

```bash
# ルートディレクトリから全サービスをビルド・起動
cargo build
cargo run --bin authservice
cargo run --bin gameservice  
cargo run --bin bff

# ProfileServiceは別ターミナルで
cd backend/profileservice
go run cmd/main.go
```

## 開発

### プロジェクト構造

```
ikutio-backend/
├── Cargo.toml                 # Rustワークスペース設定
├── docker-compose.yaml        # データベース設定
├── backend/
│   ├── authservice/           # 認証サービス (Rust)
│   │   ├── proto/auth.proto
│   │   └── src/
│   ├── profileservice/        # プロファイルサービス (Go)
│   │   ├── proto/profile.proto
│   │   ├── cmd/main.go
│   │   └── Makefile
│   ├── gameservice/           # ゲームサービス (Rust)
│   │   └── src/
│   ├── bff/                   # BFF API Gateway (Rust)
│   │   └── src/
│   ├── proto-builder/         # プロトファイルビルダー (Rust)
│   └── db/                    # データベース設定
│       ├── mysql/
│       ├── postgresql/
│       └── dynamodb/
└── target/                    # ビルド成果物
```

### API仕様

#### AuthService (gRPC)
- `Login(email, password)` - ユーザーログイン
- `RefreshLogin(refresh_token)` - トークンリフレッシュ

#### ProfileService (gRPC)  
- `CreateProfile(name)` - プロファイル作成

### 開発フロー

1. **新機能開発**
   ```bash
   # フィーチャーブランチを作成
   git checkout -b feature/new-feature
   
   # プロトファイルを更新（必要に応じて）
   # backend/*/proto/*.proto を編集
   
   # Rustサービスの場合
   cargo build
   cargo test
   
   # Goサービスの場合
   cd backend/profileservice
   make all
   go test ./...
   ```

2. **データベーススキーマ変更**
   ```bash
   # マイグレーションファイルを作成
   # backend/db/mysql/migrations/ または
   # backend/db/postgresql/migrations/ に追加
   ```

3. **テスト実行**
   ```bash
   # Rustサービス
   cargo test
   
   # Goサービス
   cd backend/profileservice
   go test ./...
   ```

## 環境設定

### データベース環境変数

各データベースの環境変数は以下のファイルで設定：

- MySQL: `backend/db/mysql/dev/.env`
- PostgreSQL: `backend/db/postgresql/dev/.env`

### サービス設定

各サービスの設定ファイル：
- AuthService: `backend/authservice/src/`
- ProfileService: `backend/profileservice/cmd/`
- GameService: `backend/gameservice/src/`
- BFF: `backend/bff/src/`

## Docker

### データベースのみ起動
```bash
docker compose up -d mysql postgresql
```

### 全サービスのコンテナ化（開発中）
各サービスディレクトリにDockerfileが配置されています。

## トラブルシューティング

### よくある問題

1. **protocが見つからない**
   ```bash
   brew install protobuf  # macOS
   sudo apt-get install protobuf-compiler  # Ubuntu
   ```

2. **Cargoビルドエラー**
   ```bash
   cargo clean
   cargo build
   ```

3. **データベース接続エラー**
   ```bash
   docker compose down
   docker compose up -d
   ```

4. **ポート競合**
   - MySQL: 3306
   - PostgreSQL: 5432
   - gRPCサービス: 各サービスのポート設定を確認

## コントリビューション

1. フィーチャーブランチを作成
2. 変更を実装
3. テストを追加・実行
4. プルリクエストを作成

## ライセンス

プライベートプロジェクト

## 関連リンク

- [Protocol Buffers](https://developers.google.com/protocol-buffers)
- [gRPC](https://grpc.io/)
- [Rust](https://www.rust-lang.org/)
- [Go](https://golang.org/)