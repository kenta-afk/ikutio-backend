package infrastructure

import (
	"context"
	"database/sql"

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

func (r *ProfileRepositoryImpl) FindById(ctx context.Context, id models.UserId) (*models.Profile, error) {
	row := r.db.QueryRowContext(ctx, "SELECT id, name, created_at, updated_at FROM profiles WHERE id = ?", id)
	var profile models.Profile

	err := row.Scan(&profile.ID, &profile.Name, &profile.CreatedAt, &profile.UpdatedAt)
	if err != nil {
		dbErr := ConvertSqlError(err)
		return nil, ConvertDbErrorToProfileError(dbErr)
	}
	return &profile, nil
}
