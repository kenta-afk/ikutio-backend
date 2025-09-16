pub enum LocationError {
    AlreadyExists,
    NotFoundRecord,
    InvalidInput,
    InternalError(String),
}
