use axum::http::StatusCode;
/// for impl IntoResponse for grpc response errors
use axum::response::{IntoResponse, Response};

pub type AppResult<T> = Result<T, AppError>;

pub struct AppError(Response);

impl AppError {
    pub fn internal_error(message: impl std::fmt::Display) -> Self {
        Self((StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response())
    }

    #[allow(dead_code)]
    pub fn bad_request(message: impl std::fmt::Display) -> Self {
        Self((StatusCode::BAD_REQUEST, message.to_string()).into_response())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.0
    }
}
