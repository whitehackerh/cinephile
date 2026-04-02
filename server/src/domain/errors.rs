use thiserror::Error;

#[derive(Debug, Error, Clone)] // Cloneがあると扱いやすいです
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