pub enum AuthError {
    UserNotFound,
    FailedHashError,
    InvalidPassword,
    DatabaseError,
    ConfigurationError,
    InternalError,
}
