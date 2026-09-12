use axum::{
    extract::{Extension, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    presentation::presenters::{
        base_response::ApiResponse,
        get_reviews::GetReviewsPresenter
    },
    usecases::{
        dto::get_reviews::GetReviewsInput,
        port::get_reviews::GetReviewsUseCase
    }
};

pub async fn get_reviews_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn GetReviewsUseCase + Send + Sync>>,
) -> impl IntoResponse {
    match usecase.execute(GetReviewsInput {
        user_id: auth_user.id(),
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), GetReviewsPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
