package domain

import (
	"context"

	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

type ProfileRepository interface {
	Save(ctx context.Context, profile *models.Profile) error
}
