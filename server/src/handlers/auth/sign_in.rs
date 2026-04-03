use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::usecases::dto::sign_in::SignInInput;
use crate::usecases::port::sign_in::SignInUseCase;
use crate::handlers::auth::schema::SignInRequest;
use crate::handlers::schema::ApiResponse;

pub async fn signin_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn SignInUseCase + Send + Sync>>,
    Json(payload): Json<SignInRequest>,
) -> impl IntoResponse {
    let input = SignInInput {
        email: payload.email,
        password: payload.password,
    };

    match usecase.execute(input).await {
        Ok(output) => {
            let mut headers = HeaderMap::new();
            let bearer_token = format!("Bearer {}", output.token);

            headers.insert(
                header::AUTHORIZATION,
                bearer_token.parse().unwrap(),
            );

            (
                StatusCode::OK,
                headers,
                Json(ApiResponse::<Value>::success(uri.to_string(), json!({})))
            )
            .into_response()
        }
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use std::sync::Arc;
    use crate::domain::errors::AppError;
    use crate::usecases::dto::sign_in::{SignInInput, SignInOutput};
    use crate::usecases::port::sign_in::SignInUseCase;

    struct MockSignInUseCase {
        result: Result<String, AppError>,
    }

    #[async_trait::async_trait]
    impl SignInUseCase for MockSignInUseCase {
        async fn execute(&self, _input: SignInInput) -> Result<SignInOutput, AppError> {
            self.result.clone().map(|token| SignInOutput { token })
        }
    }

    fn build_router(usecase: Arc<dyn SignInUseCase + Send + Sync>) -> Router {
        Router::new()
            .route("/signin", post(signin_handler))
            .with_state(usecase)
    }

    async fn read_body(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn handler_returns_200_and_authorization_header_on_success() {
        let usecase: Arc<dyn SignInUseCase + Send + Sync> = Arc::new(MockSignInUseCase {
            result: Ok("my.jwt.token".to_string()),
        });
        let app = build_router(usecase);

        let req = Request::builder()
            .method("POST")
            .uri("/signin")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"user@example.com","password":"pass123"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let auth_header = response.headers().get("authorization").unwrap();
        assert_eq!(auth_header, "Bearer my.jwt.token");
    }

    #[tokio::test]
    async fn handler_returns_401_when_unauthorized() {
        let usecase: Arc<dyn SignInUseCase + Send + Sync> = Arc::new(MockSignInUseCase {
            result: Err(AppError::Unauthorized("Invalid email or password".into())),
        });
        let app = build_router(usecase);

        let req = Request::builder()
            .method("POST")
            .uri("/signin")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"user@example.com","password":"wrong"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().get("authorization").is_none());

        let body = read_body(response.into_body()).await;
        assert_eq!(body["error"]["code"], "UNAUTHORIZED");
    }

    #[tokio::test]
    async fn handler_returns_422_for_invalid_json_body() {
        let usecase: Arc<dyn SignInUseCase + Send + Sync> = Arc::new(MockSignInUseCase {
            result: Ok("token".to_string()),
        });
        let app = build_router(usecase);

        let req = Request::builder()
            .method("POST")
            .uri("/signin")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"not_email": "missing_password_field"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        // Axum returns 422 for missing required fields in JSON
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn handler_returns_500_when_infrastructure_error() {
        let usecase: Arc<dyn SignInUseCase + Send + Sync> = Arc::new(MockSignInUseCase {
            result: Err(AppError::Infrastructure("db failure".into())),
        });
        let app = build_router(usecase);

        let req = Request::builder()
            .method("POST")
            .uri("/signin")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"user@example.com","password":"pass"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = read_body(response.into_body()).await;
        assert_eq!(body["error"]["code"], "INTERNAL_SERVER_ERROR");
        // Infrastructure error message should be hidden
        assert_eq!(body["error"]["message"], "Internal server error");
    }

    #[tokio::test]
    async fn handler_response_body_contains_uri_on_success() {
        let usecase: Arc<dyn SignInUseCase + Send + Sync> = Arc::new(MockSignInUseCase {
            result: Ok("token123".to_string()),
        });
        let app = build_router(usecase);

        let req = Request::builder()
            .method("POST")
            .uri("/signin")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"user@example.com","password":"pass"}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = read_body(response.into_body()).await;
        assert_eq!(body["uri"], "/signin");
    }
}