use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::json;

use crate::{
    generated::api_schema::SignUpRequest,
    presentation::presenters::base_response::ApiResponse,
    usecases::{
        dto::sign_up::SignUpInput,
        port::sign_up::SignUpUseCase
    }
};

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
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::success(uri.to_string(), json!({})))).into_response(),
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
