use axum::{
    extract::{Extension, Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::{
        PatchReviewsRequest,
        ReviewsPathParam
    },
    presentation::presenters::{
        base_response::ApiResponse,
        patch_reviews::PatchReviewsPresenter
    },
    usecases::{
        dto::patch_reviews::PatchReviewsInput,
        port::patch_reviews::PatchReviewsUseCase
    }
};

pub async fn patch_reviews_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn PatchReviewsUseCase + Send + Sync>>,
    Path(path): Path<ReviewsPathParam>,
    Json(payload): Json<PatchReviewsRequest>
) -> impl IntoResponse {
    match usecase.execute(PatchReviewsInput {
        id: path.id,
        user_id: auth_user.id(),
        rating: payload.rating as i32,
        content: payload.content,
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), PatchReviewsPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
