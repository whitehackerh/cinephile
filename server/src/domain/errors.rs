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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_error_message_format() {
        let err = AppError::Unauthorized("Invalid email or password".into());
        assert_eq!(err.to_string(), "Unauthorized: Invalid email or password");
    }

    #[test]
    fn unauthorized_error_is_cloneable() {
        let err = AppError::Unauthorized("test".into());
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }

    #[test]
    fn unauthorized_error_with_empty_message() {
        let err = AppError::Unauthorized(String::new());
        assert_eq!(err.to_string(), "Unauthorized: ");
    }

    #[test]
    fn all_error_variants_have_correct_messages() {
        let cases = vec![
            (AppError::EntityNotFound("user".into()), "Entity not found: user"),
            (AppError::AlreadyExists("email".into()), "Already exists: email"),
            (AppError::Validation("bad input".into()), "Validation error: bad input"),
            (AppError::Infrastructure("db down".into()), "Infrastructure error: db down"),
            (AppError::Unauthorized("no access".into()), "Unauthorized: no access"),
        ];
        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected);
        }
    }

    #[test]
    fn unauthorized_debug_contains_message() {
        let err = AppError::Unauthorized("forbidden".into());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Unauthorized"));
        assert!(debug_str.contains("forbidden"));
    }
}