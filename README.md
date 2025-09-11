# Ikutio Backend

ハッカソン「イクチオカップ」のバックエンドリポジトリです。

## アーキテクチャ

このプロジェクトはマイクロサービスアーキテクチャを採用しており、以下のような構成になっています：

```
ikutio-backend/
├── backend/
│   ├── bff/                    # Backend for Frontend (Rust + Axum)
│   ├── authservice/            # 認証サービス (Rust + tonic)
│   ├── gameservice/            # ゲームロジックサービス (Rust)
│   ├── profileservice/         # プロフィール管理サービス (Go + grpc-go)
│   ├── proto-builder/          # Protocol Buffers コード生成ツール
│   └── db/                     # データベース設定
│       ├── mysql/              # MySQL設定・マイグレーション
│       ├── postgresql/         # PostgreSQL設定・マイグレーション
│       └── dynamodb/           # DynamoDB設定
└── README.md
```

## 技術スタック

- **BFF**: Rust + Axum (REST API提供)
- **認証サービス**: Rust + tonic + PostgreSQL
- **ゲームサービス**: Rust + tonic
- **プロフィールサービス**: Go + grpc-go + PostgreSQL（予定）
- **通信プロトコル**: gRPC (Protocol Buffers)
- **データベース**: PostgreSQL, MySQL, DynamoDB
- **コンテナ化**: Docker + Docker Compose
- **コード生成**: tonic-prost-build

## セットアップ

### 前提条件

- Rust (最新版)
- Go 1.21+ (profileservice開発時)
- Docker & Docker Compose
- Protocol Buffers コンパイラ
  ```bash
  # macOS
  brew install protobuf
  
  # Ubuntu/Debian
  sudo apt install protobuf-compiler
  ```

### プロジェクトのビルド

1. リポジトリをクローン
   ```bash
   git clone https://github.com/kenta-afk/ikutio-backend.git
   cd ikutio-backend
   ```

2. Docker環境での開発（推奨）
   ```bash
   # 全サービスを起動
   docker-compose up -d
   
   # 特定のサービスのみ起動
   docker-compose up -d postgresql mysql authservice bff
   ```

3. ローカル開発の場合
   ```bash
   # Protocol Buffersコードを生成
   cd backend/proto-builder
   cargo run
   
   # 各サービスをビルド
   cd ../bff
   cargo build
   
   cd ../authservice
   cargo build
   ```

## Protocol Buffers コード生成

`proto-builder`は各サービスのProtocol Buffersファイルから、適切なクライアント・サーバーコードを自動生成します：

- **BFF用**: クライアントコードのみ生成 (`bff/src/proto/`)
- **各サービス用**: サーバーコードのみ生成 (`{service}/src/proto/`)

### 新しいサービスの追加

1. `backend/` ディレクトリに新しいサービスディレクトリを作成
2. `proto/` ディレクトリに `.proto` ファイルを配置
3. `proto-builder/src/main.rs` の `services` 配列に追加
4. `cargo run` でコード生成

例：
```rust
let services = vec![
    ("authservice", "auth"),
    ("gameservice", "game"),
    ("profileservice", "profile"),
];
```

## 開発

### サービス構成

#### BFF (Backend for Frontend)
- **技術**: Rust + Axum
- **役割**: REST APIエンドポイントの提供、マイクロサービスへのルーティング
- **ポート**: 50052

#### 認証サービス (authservice)
- **技術**: Rust + tonic + PostgreSQL
- **役割**: ユーザー認証、JWT発行・検証
- **ポート**: 50053

#### プロフィールサービス (profileservice)
- **技術**: Go + grpc-go + PostgreSQL
- **役割**: ユーザープロフィール管理
- **ポート**: 50054（予定）

#### ゲームサービス (gameservice)
- **技術**: Rust + tonic
- **役割**: ゲームロジック処理
- **ポート**: 未定

### BFF開発

BFFはRust + Axumを使用し、各マイクロサービスへのgRPCクライアントとして動作します。REST APIやGraphQLエンドポイントを提供します。

### マイクロサービス開発

各マイクロサービスは異なる言語で実装可能です：

- **Rust**: tonicを使用
- **Go**: grpc-goを使用  
- **Python**: grpcio-toolsを使用
- **Node.js**: @grpc/grpc-jsを使用

## データベース構成

### PostgreSQL
- **用途**: メインデータベース（ユーザー情報、認証情報）
- **ポート**: 5432
- **マイグレーション**: SQLXを使用

### MySQL
- **用途**: 補助データベース
- **ポート**: 3306

### DynamoDB Local
- **用途**: NoSQLデータ（セッション、キャッシュ等）
- **ポート**: 8000
- **管理画面**: 8001 (dynamodb-admin)

## 開発環境

### Docker Compose サービス
- `postgresql` - PostgreSQLデータベース
- `mysql` - MySQLデータベース
- `dynamodb-local` - DynamoDB Local
- `dynamodb-admin` - DynamoDB管理画面
- `migration` - PostgreSQLマイグレーション
- `authservice` - 認証サービス
- `bff` - Backend for Frontend

### 環境変数
各サービスの設定は以下のファイルで管理：
- `backend/authservice/dev/.env`
- `backend/bff/dev/.env`
- `backend/db/postgresql/dev/.env`
- `backend/db/mysql/dev/.env`
- `backend/db/dynamodb/dev/.env`

## プロジェクト構造の利点

- **マイクロサービスアーキテクチャ**: サービス単位での独立した開発・デプロイ
- **言語の多様性**: 各サービスで最適な言語を選択可能（Rust、Go等）
- **スケーラビリティ**: サービス単位での独立したスケーリング
- **開発効率**: Protocol Buffersによる型安全な通信
- **保守性**: 責務の明確な分離
- **Docker化**: 環境の統一と簡単なセットアップ

## コントリビューション

1. ブランチを作成
2. 変更を実装
3. Protocol Buffersを更新した場合は `proto-builder` を実行
4. プルリクエストを作成

## ライセンス

[ライセンス情報をここに記載]

---

**イクチオカップ 2025** 🐟🏆
