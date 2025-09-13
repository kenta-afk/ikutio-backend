package application

import (
	"context"
	"log/slog"

	"github.com/google/uuid"
	pb "github.com/kenta-afk/ikutio-backend/build"
	"github.com/kenta-afk/ikutio-backend/internal/application/commands"
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

// ProfileGRPCService は ProfileServiceServer インターフェースの実装
type ProfileGRPCService struct {
	// UnimplementedProfileServiceServer を埋め込むことで、
	// 将来追加されるメソッドとの前方互換性を保つ
	pb.UnimplementedProfileServiceServer
	profileService *ProfileServiceImpl
}

// NewProfileGRPCService は新しい ProfileGRPCService インスタンスを作成
func NewProfileGRPCService(profileService *ProfileServiceImpl) *ProfileGRPCService {
	return &ProfileGRPCService{
		profileService: profileService,
	}
}

// CreateProfile は CreateProfile RPC メソッドの実装
func (s *ProfileGRPCService) CreateProfile(ctx context.Context, req *pb.CreateProfileRequest) (*pb.CreateProfileReply, error) {
	slog.Info("CreateProfile called", "name", req.Name)

	// 既存のProfileServiceImplのCreateProfileを使用
	cmd := commands.CreateProfileCommand{
		Id:   models.UserId(uuid.New()), // UUIDを生成してUserIdにキャスト
		Name: req.Name,
	}

	err := s.profileService.CreateProfile(ctx, cmd)
	if err != nil {
		slog.Error("Failed to create profile", "error", err)
		return nil, err
	}

	// 成功時は作成されたプロファイル情報を返す
	return &pb.CreateProfileReply{
		Name: req.Name,
	}, nil
}
