use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AppError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}