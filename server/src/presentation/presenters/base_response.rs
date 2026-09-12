use axum::{
    Json,
    http::{
        StatusCode, 
        Uri
    }
};
use chrono::Utc;
use serde::Serialize;

use crate::domain::errors::AppError;
pub use crate::generated::api_schema::{ApiErrorDetail, ApiResponse};

impl ApiResponse {
    pub fn success<T: Serialize>(uri: String, data: T) -> Self {
        let value = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);

        let data = serde_json::from_value(value)
            .unwrap_or_else(|_| serde_json::from_value(serde_json::Value::Null).unwrap());

        Self {
            uri,
            timestamp: Utc::now(),
            data,
            error: None,
        }
    }

    pub fn error(uri: String, code: &str, message: &str) -> Self {
        let data = serde_json::from_value(serde_json::Value::Null)
            .expect("Failed to deserialize Value::Null into ApiResponseData");

        Self {
            uri,
            timestamp: Utc::now(),
            data: data,
            error: Some(ApiErrorDetail {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }

    pub fn from_error(uri: &Uri, err: AppError) -> (StatusCode, Json<ApiResponse>) {
        let (status, code) = match &err {
            AppError::EntityNotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Conflict(_) => (StatusCode::CONFLICT, "CONFLICT"),
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