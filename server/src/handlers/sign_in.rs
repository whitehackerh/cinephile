use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};
use crate::usecases::dto::sign_in::SignInInput;
use crate::usecases::port::sign_in::SignInUseCase;
use crate::handlers::base_response::ApiResponse;

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

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
