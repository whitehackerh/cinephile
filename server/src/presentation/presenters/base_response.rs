use axum::{
    Json,
    http::{
        StatusCode, 
        Uri
    }
};
use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use crate::domain::errors::AppError;
pub use crate::generated::api_schema::{ApiErrorDetail, ApiResponse};

impl ApiResponse {
    pub fn success<T: Serialize>(uri: String, data: T) -> Self {
        let map_data = serde_json::to_value(data)
            .ok()
            .and_then(|v| match v {
                Value::Object(map) => Some(map),
                _ => None,
            });

        Self {
            uri,
            timestamp: Utc::now(),
            data: map_data,
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

    pub fn from_error(uri: &Uri, err: AppError) -> (StatusCode, Json<ApiResponse>) {
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

        (
            status,
            Json(ApiResponse::error(uri.to_string(), code, &message)),
        )
    }
}