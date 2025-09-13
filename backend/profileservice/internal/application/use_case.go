package application

import (
	"context"

	"github.com/kenta-afk/ikutio-backend/internal/application/commands"
	"github.com/kenta-afk/ikutio-backend/internal/application/dtos"
	"github.com/kenta-afk/ikutio-backend/internal/domain"
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

type ProfileServiceImpl struct {
	repo domain.ProfileRepository
}

func New(repo domain.ProfileRepository) *ProfileServiceImpl {
	return &ProfileServiceImpl{
		repo: repo,
	}
}

func (s *ProfileServiceImpl) CreateProfile(ctx context.Context, cmd commands.CreateProfileCommand) (*dtos.CreateProfileDto, error) {
	profile := models.New(cmd.Id, cmd.Name)
	err := s.repo.Save(ctx, profile)
	if err != nil {
		return nil, err
	}

	// DTOを作成して返す
	responseDto := &dtos.CreateProfileDto{
		Name: profile.Name,
	}

	return responseDto, nil
}
