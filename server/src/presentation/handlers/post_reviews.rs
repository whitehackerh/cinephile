use axum::{
    extract::{Extension, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::PostReviewsRequest,
    presentation::presenters::{
        base_response::ApiResponse,
        post_reviews::PostReviewsPresenter
    },
    usecases::{
        dto::post_reviews::PostReviewsInput,
        port::post_reviews::PostReviewsUseCase
    }
};

pub async fn post_reviews_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn PostReviewsUseCase + Send + Sync>>,
    Json(payload): Json<PostReviewsRequest>,
) -> impl IntoResponse {
    match usecase.execute(PostReviewsInput {
        user_id: auth_user.id(),
        rating: payload.rating as i32,
        content: payload.content,
        work_type: payload.work_type.to_string(),
        target_path: payload.target_path
    }).await {
        Ok(output) => {
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(uri.to_string(), PostReviewsPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
