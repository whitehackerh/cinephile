use axum::{response::{IntoResponse, Response}, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        (status, self.to_string()).into_response()
    }
}
