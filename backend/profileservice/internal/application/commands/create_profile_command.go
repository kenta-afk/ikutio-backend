package commands

import (
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

type CreateProfileCommand struct {
	Id   models.UserId
	Name string
}