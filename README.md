# Ikutio Backend

ハッカソン「イクチオカップ」のバックエンドリポジトリです。

## アーキテクチャ

このプロジェクトはマイクロサービスアーキテクチャを採用しており、以下のような構成になっています：

```
ikutio-backend/
├── backend/
│   ├── bff/                    # Backend for Frontend (Rust + Axum)
│   ├── services/               # マイクロサービス群
│   │   ├── authservice/        # 認証サービス (Rust)
│   │   ├── gameservice/        # ゲームロジックサービス
│   │   └── profileservice/     # プロフィール管理サービス
│   └── proto-builder/          # Protocol Buffers コード生成ツール
└── README.md
```

## 技術スタック

- **BFF**: Rust + Axum (確定)
- **マイクロサービス**: 言語自由（Rust、Go、Python、Node.js など）
- **通信プロトコル**: gRPC (Protocol Buffers)
- **コード生成**: tonic-prost-build

## セットアップ

### 前提条件

- Rust (最新版)
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

2. Protocol Buffersコードを生成
   ```bash
   cd backend/proto-builder
   cargo run
   ```

3. 各サービスをビルド
   ```bash
   # BFFをビルド
   cd ../bff
   cargo build
   
   # authserviceをビルド
   cd ../services/authservice
   cargo build
   ```

## Protocol Buffers コード生成

`proto-builder`は各サービスのProtocol Buffersファイルから、適切なクライアント・サーバーコードを自動生成します：

- **BFF用**: クライアントコードのみ生成 (`bff/src/services/`)
- **各サービス用**: サーバーコードのみ生成 (`services/{service}/proto/`)

### 新しいサービスの追加

1. `services/` ディレクトリに新しいサービスディレクトリを作成
2. `proto/` ディレクトリに `.proto` ファイルを配置
3. `proto-builder/src/main.rs` の `services` 配列に追加
4. `cargo run` でコード生成

例：
```rust
let services = vec![
    ("authservice", "auth"),
    ("gameservice", "game"),     // 新規追加
    ("profileservice", "profile"), // 新規追加
];
```

## 開発

### BFF開発

BFFは確定でRust + Axumを使用します。各マイクロサービスへのgRPCクライアントとして動作し、REST APIやGraphQLエンドポイントを提供します。

### マイクロサービス開発

各マイクロサービスは言語を自由に選択できます：

- **Rust**: tonicを使用
- **Go**: grpc-goを使用  
- **Python**: grpcio-toolsを使用
- **Node.js**: @grpc/grpc-jsを使用

## プロジェクト構造の利点

- **言語の自由度**: BFF以外は好きな言語で開発可能
- **スケーラビリティ**: サービス単位での独立したスケーリング
- **開発効率**: 自動コード生成により型安全な通信
- **保守性**: 責務の明確な分離

## コントリビューション

1. ブランチを作成
2. 変更を実装
3. Protocol Buffersを更新した場合は `proto-builder` を実行
4. プルリクエストを作成

## ライセンス

[ライセンス情報をここに記載]

---

**イクチオカップ 2025** 🐟🏆
