package infrastructure

import (
	"context"
	"database/sql"

	"github.com/google/uuid"
	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

type ProfileRepositoryImpl struct {
	db *sql.DB
}

func New(db *sql.DB) *ProfileRepositoryImpl {
	return &ProfileRepositoryImpl{
		db: db,
	}
}

func (r *ProfileRepositoryImpl) Save(ctx context.Context, profile *models.Profile) error {
	_, err := r.db.ExecContext(ctx, "INSERT INTO profiles (id, name) VALUES (?, ?)",
		profile.ID, profile.Name)
	if err != nil {
		dbErr := ConvertSqlError(err)
		return ConvertDbErrorToProfileError(dbErr)
	}
	return nil
}

func (r *ProfileRepositoryImpl) FindByUserId(ctx context.Context, userId models.UserId) (*models.Profile, error) {
	var profile models.Profile
	err := r.db.QueryRowContext(ctx, "SELECT id, name FROM profiles WHERE id = ?", uuid.UUID(userId).String()).
		Scan(&profile.ID, &profile.Name)

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, models.NewProfileErrorWithoutCause(models.ProfileErrorNotFound, "profile not found")
		}
		dbErr := ConvertSqlError(err)
		return nil, ConvertDbErrorToProfileError(dbErr)
	}

	return &profile, nil
}