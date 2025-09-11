use crate::infrastructure::error::DbError;

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Configuration(source) => DbError::Configuration(source),
            sqlx::Error::InvalidArgument(msg) => DbError::InvalidArgument(msg),
            sqlx::Error::Database(db_err) => DbError::Database(db_err),
            sqlx::Error::Io(io_err) => DbError::Io(io_err),
            sqlx::Error::RowNotFound => DbError::RowNotFound,
            sqlx::Error::TypeNotFound { type_name } => DbError::TypeNotFound { type_name },
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                DbError::ColumnIndexOutOfBounds { index, len }
            }
            sqlx::Error::ColumnNotFound(name) => DbError::ColumnNotFound(name),
            sqlx::Error::ColumnDecode { index, source } => DbError::ColumnDecode { index, source },
            sqlx::Error::Encode(source) => DbError::Encode(source),
            sqlx::Error::Decode(source) => DbError::Decode(source),
            sqlx::Error::AnyDriverError(source) => DbError::AnyDriverError(source),
            sqlx::Error::WorkerCrashed => DbError::WorkerCrashed,
            _ => DbError::InvalidArgument(err.to_string()),
        }
    }
}
