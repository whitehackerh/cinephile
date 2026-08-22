use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::json;

use crate::{
    generated::api_schema::SignInRequest,
    presentation::presenters::base_response::ApiResponse,
    usecases::{
        dto::sign_in::SignInInput,
        port::sign_in::SignInUseCase
    }
};

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
                Json(ApiResponse::success(uri.to_string(), json!({})))
            )
            .into_response()
        }
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
