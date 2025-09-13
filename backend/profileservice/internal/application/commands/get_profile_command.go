package commands

import (
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

type GetProfileCommand struct {
	Id models.UserId
}