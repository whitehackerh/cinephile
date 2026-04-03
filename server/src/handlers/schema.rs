use axum::http::{StatusCode, Uri};
use axum::Json;
use serde::Serialize;
use serde_json::Value;
use chrono::{DateTime, Utc};
use crate::domain::errors::AppError;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub uri: String,
    pub timestamp: DateTime<Utc>,
    pub data: Option<T>,
    pub error: Option<ApiErrorDetail>,
}

#[derive(Serialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(uri: String, data: T) -> Self {
        Self {
            uri,
            timestamp: Utc::now(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(uri: String, code: &str, message: &str) -> Self {
        Self {
            uri,
            timestamp: Utc::now(),
            data: None,
            error: Some(ApiErrorDetail {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }

    pub fn from_error(uri: &Uri, err: AppError) -> (StatusCode, Json<ApiResponse<Value>>) {
        let (status, code) = match &err {
            AppError::EntityNotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::AlreadyExists(_) => (StatusCode::CONFLICT, "ALREADY_EXISTS"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::Infrastructure(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
        };

        let message = match &err {
            AppError::Infrastructure(_) => "Internal server error".to_string(),
            _ => err.to_string(),
        };

        (status, Json(ApiResponse::<Value>::error(
            uri.to_string(),
            code,
            &message
        )))
    }
}
