use tonic::Status;

use crate::internal::domain::models::error::LocationError;
use crate::internal::infrastructure::error::DbError;

impl From<LocationError> for Status {
    fn from(error: LocationError) -> Self {
        match error {
            LocationError::AlreadyExists => Status::already_exists("Record already exists"),
            LocationError::NotFoundRecord => Status::not_found("Record not found"),
            LocationError::InvalidInput => Status::invalid_argument("Invalid input"),
            LocationError::InternalError(_) => Status::internal("Internal server error"),
        }
    }
}

impl From<DbError> for LocationError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Conflict(_) => LocationError::AlreadyExists,
            DbError::NotFound(_) => LocationError::NotFoundRecord,
            DbError::Infrastructure(msg) => LocationError::InternalError(msg),
        }
    }
}
