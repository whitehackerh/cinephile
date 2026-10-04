use axum::{
    extract::{Extension, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::PostReviewRequest,
    presentation::presenters::{
        base_response::ApiResponse,
        create_review::CreateReviewPresenter
    },
    usecases::{
        dto::create_review::CreateReviewInput,
        port::create_review::CreateReviewUseCase
    }
};

pub async fn create_review_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn CreateReviewUseCase + Send + Sync>>,
    Json(payload): Json<PostReviewRequest>,
) -> impl IntoResponse {
    match usecase.execute(CreateReviewInput {
        user_id: auth_user.id(),
        rating: payload.rating as i32,
        content: payload.content,
        work_type: payload.work_type.to_string(),
        target_path: payload.target_path
    }).await {
        Ok(output) => {
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(uri.to_string(), CreateReviewPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
