# Profile Service

プロファイルサービスはユーザーのプロファイル情報を管理するgRPCサービスです。

## 前提条件

- Go 1.19以上
- Protocol Buffers Compiler (protoc)
- Rust (Cargoツールチェーン)

## Makefileの使い方

このディレクトリには、プロトコルバッファファイルの生成とプロジェクトのセットアップを自動化するMakefileが含まれています。

### 利用可能なコマンド

#### `make all` (デフォルトターゲット)
すべてのセットアップとビルドを実行します。
```bash
make all
```

このコマンドは以下の処理を行います：
1. 必要なツールの設定（`make setup`を実行）
2. プロトコルバッファファイルからGoのサーバーコードを生成
3. プロトコルバッファファイルからRustのクライアントコードを生成（BFFサービス用）

#### `make setup`
必要な依存関係とツールをインストールします。
```bash
make setup
```

このコマンドは以下を実行します：
- Go modulesの整理（`go mod tidy`）
- protoc-gen-goプラグインのインストール
- protoc-gen-go-grpcプラグインのインストール  
- protoc-gen-prostプラグインのインストール（Rust用）
- protoc-gen-tonicプラグインのインストール（Rust用）

#### `make clean`
ビルド成果物をクリーンアップします。
```bash
make clean
```

このコマンドは以下を削除します：
- `build/`ディレクトリとその中身
- BFFサービスディレクトリ内の生成されたRustファイル
- Goのビルドキャッシュ

### 典型的な使用例

#### 初回セットアップ
```bash
# 初回は依存関係のインストールから
make setup

# プロトコルバッファファイルの生成
make all
```

#### プロトファイル変更後の再生成
```bash
# クリーンアップしてから再生成
make clean
make all
```

#### 依存関係の再インストール
```bash
make setup
```

### 生成されるファイル

- **Go用（サーバー側）**: `build/`ディレクトリ内
  - `profile.pb.go` - プロトコルバッファメッセージ
  - `profile_grpc.pb.go` - gRPCサーバー/クライアントコード

- **Rust用（BFFクライアント側）**: `../bff/src/services/`ディレクトリ内
  - `profile.rs` - プロトコルバッファメッセージ
  - `profile_grpc.rs` - gRPCクライアントコード

### トラブルシューティング

#### protocが見つからない場合
```bash
# macOSの場合
brew install protobuf

# Ubuntuの場合
sudo apt-get install protobuf-compiler
```

#### Goツールのインストールに失敗する場合
```bash
# GOPATHとGOROOTが正しく設定されているか確認
go env GOPATH
go env GOROOT

# 手動でツールをインストール
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

#### Rustツールのインストールに失敗する場合
```bash
# Rustが正しくインストールされているか確認
rustc --version
cargo --version

# 手動でツールをインストール
cargo install protoc-gen-prost
cargo install protoc-gen-tonic
```

## プロジェクト構造

```
profileservice/
├── Makefile           # ビルド自動化
├── README.md          # このファイル
├── main.go           # メインサーバーファイル
├── go.mod            # Go modules設定
├── go.sum            # Go依存関係チェックサム
├── proto/            # プロトコルバッファ定義
│   └── profile.proto
└── build/            # 生成されるGoコード
    ├── profile.pb.go
    └── profile_grpc.pb.go
```

## 開発フロー

1. `proto/profile.proto`ファイルを編集
2. `make clean && make all`で新しいコードを生成
3. `main.go`や他のGoファイルでサーバーロジックを実装
4. テストとデバッグを実行
