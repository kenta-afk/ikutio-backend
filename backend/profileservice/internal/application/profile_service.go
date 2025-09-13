package application

import (
	"context"
	"errors"
	"log/slog"

	"github.com/google/uuid"
	pb "github.com/kenta-afk/ikutio-backend/build"
	"github.com/kenta-afk/ikutio-backend/internal/application/commands"
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
	"google.golang.org/grpc/metadata"
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

	// metadataからuser_idを取得
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		slog.Error("Failed to get metadata from context")
		return nil, errors.New("failed to get metadata from context")
	}

	userIdStrings := md.Get("user-id")
	if len(userIdStrings) == 0 {
		slog.Error("user-id not found in metadata")
		return nil, errors.New("user-id not found in metadata")
	}

	userIdString := userIdStrings[0]
	userId, err := uuid.Parse(userIdString)
	if err != nil {
		slog.Error("Failed to parse user_id from metadata", "user_id", userIdString, "error", err)
		return nil, err
	}

	// 既存のProfileServiceImplのCreateProfileを使用
	cmd := commands.CreateProfileCommand{
		Id:   models.UserId(userId), // metadataから取得したuser_idを使用
		Name: req.Name,
	}

	dto, err := s.profileService.CreateProfile(ctx, cmd)
	if err != nil {
		slog.Error("Failed to create profile", "error", err)
		return nil, err
	}

	// 成功時は作成されたプロファイル情報を返す
	return &pb.CreateProfileReply{
		Name: dto.Name,
	}, nil
}
