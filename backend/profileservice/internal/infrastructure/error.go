package infrastructure

import (
	"database/sql"
	"errors"
	"strings"

	"github.com/kenta-afk/ikutio-backend/internal/domain/models"
)

// DbErrorType はデータベースエラーの種類を表す
type DbErrorType int

const (
	DbErrorUnknown DbErrorType = iota
	DbErrorNotFound
	DbErrorDuplicate
	DbErrorConnection
	DbErrorConstraint
)

// DbError はinfrastructure層のデータベースエラー
type DbError struct {
	Type    DbErrorType
	Message string
	Cause   error
}

func (e DbError) Error() string {
	return e.Message
}

func (e DbError) Unwrap() error {
	return e.Cause
}

// ConvertSqlError はsql.Errorを DbErrorに変換
func ConvertSqlError(err error) DbError {
	if err == nil {
		return DbError{}
	}

	// sql.ErrNoRowsの場合
	if errors.Is(err, sql.ErrNoRows) {
		return DbError{
			Type:    DbErrorNotFound,
			Message: "record not found",
			Cause:   err,
		}
	}

	// MySQLの重複エラー（エラーコード1062）
	if strings.Contains(err.Error(), "Duplicate entry") {
		return DbError{
			Type:    DbErrorDuplicate,
			Message: "duplicate entry",
			Cause:   err,
		}
	}

	// その他のデータベースエラー
	return DbError{
		Type:    DbErrorUnknown,
		Message: "database error",
		Cause:   err,
	}
}

// ConvertDbErrorToProfileError はDbErrorをProfileErrorに変換
func ConvertDbErrorToProfileError(dbErr DbError) error {
	// DbErrorがゼロ値の場合（エラーなし）はnilを返す
	if dbErr.Type == 0 && dbErr.Message == "" && dbErr.Cause == nil {
		return nil
	}

	switch dbErr.Type {
	case DbErrorNotFound:
		return models.NewProfileError(
			models.ProfileErrorNotFound,
			"profile not found",
			dbErr.Cause,
		)
	case DbErrorDuplicate:
		return models.NewProfileError(
			models.ProfileErrorDuplicate,
			"profile already exists",
			dbErr.Cause,
		)
	case DbErrorConnection, DbErrorConstraint, DbErrorUnknown:
		return models.NewProfileError(
			models.ProfileErrorDatabase,
			"database operation failed",
			dbErr.Cause,
		)
	default:
		return models.NewProfileError(
			models.ProfileErrorUnknown,
			"unknown error occurred",
			dbErr.Cause,
		)
	}
}
