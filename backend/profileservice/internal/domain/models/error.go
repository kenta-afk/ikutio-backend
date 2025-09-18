package models

import "errors"

type ProfileErrorType int

const (
	ProfileErrorUnknown ProfileErrorType = iota
	ProfileErrorNotFound
	ProfileErrorDuplicate
	ProfileErrorDatabase
	ProfileErrorValidation
)

type ProfileError struct {
	Type    ProfileErrorType
	Message string
	Cause   error
}

func (e ProfileError) Error() string {
	return e.Message
}

func (e ProfileError) Unwrap() error {
	return e.Cause
}

// NewProfileError creates a new ProfileError
func NewProfileError(errorType ProfileErrorType, message string, cause error) error {
	return ProfileError{
		Type:    errorType,
		Message: message,
		Cause:   cause,
	}
}

// NewProfileErrorWithoutCause creates a new ProfileError without cause
func NewProfileErrorWithoutCause(errorType ProfileErrorType, message string) error {
	return ProfileError{
		Type:    errorType,
		Message: message,
		Cause:   nil,
	}
}

// IsProfileError checks if error is a ProfileError
func IsProfileError(err error) bool {
	var profileErr ProfileError
	return errors.As(err, &profileErr)
}

// IsProfileErrorType checks if error is a specific ProfileError type
func IsProfileErrorType(err error, errorType ProfileErrorType) bool {
	var profileErr ProfileError
	if errors.As(err, &profileErr) {
		return profileErr.Type == errorType
	}
	return false
}

// IsProfileNotFoundError checks if error is ProfileErrorNotFound
func IsProfileNotFoundError(err error) bool {
	return IsProfileErrorType(err, ProfileErrorNotFound)
}

// IsProfileDuplicateError checks if error is ProfileErrorDuplicate
func IsProfileDuplicateError(err error) bool {
	return IsProfileErrorType(err, ProfileErrorDuplicate)
}

// IsProfileDatabaseError checks if error is ProfileErrorDatabase
func IsProfileDatabaseError(err error) bool {
	return IsProfileErrorType(err, ProfileErrorDatabase)
}

// IsProfileValidationError checks if error is ProfileErrorValidation
func IsProfileValidationError(err error) bool {
	return IsProfileErrorType(err, ProfileErrorValidation)
}
