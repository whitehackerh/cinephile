use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::usecases::dto::SignUpInput;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::handlers::auth::schema::SignUpRequest;

pub async fn signup_handler(
    State(usecase): State<Arc<dyn SignUpUseCase + Send + Sync>>,
    Json(payload): Json<SignUpRequest>,
) -> impl IntoResponse {
    let input = SignUpInput {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    let result = usecase.execute(input).await;

    match result {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => e.into_response(),
    }
}
