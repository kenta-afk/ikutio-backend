#[derive(Debug)]
pub enum DbError {
    NotFound(String),
    Conflict(String),
    Infrastructure(String),
}
