use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::{json, Value};
use crate::usecases::dto::sign_up::SignUpInput;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::handlers::base_response::ApiResponse;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn signup_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn SignUpUseCase + Send + Sync>>,
    Json(payload): Json<SignUpRequest>,
) -> impl IntoResponse {
    let input = SignUpInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    match usecase.execute(input).await {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::<Value>::success(uri.to_string(), json!({})))).into_response(),
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}
