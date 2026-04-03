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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Uri;

    fn make_uri(path: &str) -> Uri {
        path.parse::<Uri>().unwrap()
    }

    #[test]
    fn from_error_unauthorized_returns_401() {
        let uri = make_uri("/signin");
        let err = AppError::Unauthorized("Invalid email or password".into());
        let (status, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);

        assert_eq!(status, StatusCode::UNAUTHORIZED);
        let error = resp.error.unwrap();
        assert_eq!(error.code, "UNAUTHORIZED");
        assert_eq!(error.message, "Unauthorized: Invalid email or password");
        assert!(resp.data.is_none());
    }

    #[test]
    fn from_error_unauthorized_preserves_uri() {
        let uri = make_uri("/api/signin");
        let err = AppError::Unauthorized("bad credentials".into());
        let (_, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);
        assert_eq!(resp.uri, "/api/signin");
    }

    #[test]
    fn from_error_infrastructure_hides_message() {
        let uri = make_uri("/signin");
        let err = AppError::Infrastructure("db connection failed with secret info".into());
        let (status, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        let error = resp.error.unwrap();
        assert_eq!(error.code, "INTERNAL_SERVER_ERROR");
        assert_eq!(error.message, "Internal server error");
    }

    #[test]
    fn from_error_entity_not_found_returns_404() {
        let uri = make_uri("/users/1");
        let err = AppError::EntityNotFound("user".into());
        let (status, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(resp.error.unwrap().code, "NOT_FOUND");
    }

    #[test]
    fn from_error_already_exists_returns_409() {
        let uri = make_uri("/signup");
        let err = AppError::AlreadyExists("Email already taken".into());
        let (status, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);

        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(resp.error.unwrap().code, "ALREADY_EXISTS");
    }

    #[test]
    fn from_error_validation_returns_400() {
        let uri = make_uri("/signup");
        let err = AppError::Validation("Name cannot be empty".into());
        let (status, Json(resp)) = ApiResponse::<Value>::from_error(&uri, err);

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(resp.error.unwrap().code, "VALIDATION_ERROR");
    }

    #[test]
    fn success_response_has_data_and_no_error() {
        let resp = ApiResponse::success("/test".to_string(), serde_json::json!({"key": "value"}));
        assert!(resp.data.is_some());
        assert!(resp.error.is_none());
        assert_eq!(resp.uri, "/test");
    }

    #[test]
    fn error_response_has_error_and_no_data() {
        let resp = ApiResponse::<Value>::error("/test".to_string(), "SOME_ERROR", "some message");
        assert!(resp.data.is_none());
        let error = resp.error.unwrap();
        assert_eq!(error.code, "SOME_ERROR");
        assert_eq!(error.message, "some message");
    }
}