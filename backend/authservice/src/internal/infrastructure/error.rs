use std::io;

use sqlx::error::{BoxDynError, DatabaseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    /// Error occurred while parsing a connection string.
    #[error("error with configuration: {0}")]
    Configuration(#[source] BoxDynError),

    /// One or more of the arguments to the called function was invalid.
    ///
    /// The string contains more information.
    #[error("{0}")]
    InvalidArgument(String),

    /// Error returned from the database.
    #[error("error returned from database: {0}")]
    Database(#[source] Box<dyn DatabaseError>),

    /// Error communicating with the database backend.
    #[error("error communicating with database: {0}")]
    Io(#[from] io::Error),

    /// No rows returned by a query that expected to return at least one row.
    #[error("no rows returned by a query that expected to return at least one row")]
    RowNotFound,

    /// Type in query doesn't exist. Likely due to typo or missing user type.
    #[error("type named {type_name} not found")]
    TypeNotFound { type_name: String },

    /// Column index was out of bounds.
    #[error("column index out of bounds: the len is {len}, but the index is {index}")]
    ColumnIndexOutOfBounds { index: usize, len: usize },

    /// No column found for the given name.
    #[error("no column found for name: {0}")]
    ColumnNotFound(String),

    /// Error occurred while decoding a value from a specific column.
    #[error("error occurred while decoding column {index}: {source}")]
    ColumnDecode {
        index: String,

        #[source]
        source: BoxDynError,
    },

    /// Error occured while encoding a value.
    #[error("error occurred while encoding a value: {0}")]
    Encode(#[source] BoxDynError),

    /// Error occurred while decoding a value.
    #[error("error occurred while decoding: {0}")]
    Decode(#[source] BoxDynError),

    /// Error occurred within the `Any` driver mapping to/from the native driver.
    #[error("error in Any driver mapping: {0}")]
    AnyDriverError(#[source] BoxDynError),

    /// A background worker has crashed.
    #[error("attempted to communicate with a crashed background worker")]
    WorkerCrashed,
}
