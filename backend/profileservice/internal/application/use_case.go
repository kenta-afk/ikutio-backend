package application

import (
	"context"

	"github.com/kenta-afk/ikutio-backend/internal/application/commands"
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

func (s *ProfileServiceImpl) CreateProfile(ctx context.Context, cmd commands.CreateProfileCommand) error {
	profile := models.New(cmd.Id, cmd.Name)
	err := s.repo.Save(ctx, profile)
	if err != nil {
		return err
	}

	return nil
}

func (s *ProfileServiceImpl) GetProfile(ctx context.Context, cmd commands.GetProfileCommand) (*models.Profile, error) {
	res, err := s.repo.FindById(ctx, cmd.Id)
	if err != nil {
		return nil, err
	}
	return res, nil
}
