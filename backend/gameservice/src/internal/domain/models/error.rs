pub enum GameError {
    AlreadyExists,
    NotFoundRecord,
    InvalidInput,
    InternalError(String),
}
