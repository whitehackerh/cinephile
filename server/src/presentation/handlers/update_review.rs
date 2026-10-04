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
        PatchReviewRequest,
        ReviewPathParam
    },
    presentation::presenters::{
        base_response::ApiResponse,
        update_review::UpdateReviewPresenter
    },
    usecases::{
        dto::update_review::UpdateReviewInput,
        port::update_review::UpdateReviewUseCase
    }
};

pub async fn update_review_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn UpdateReviewUseCase + Send + Sync>>,
    Path(path): Path<ReviewPathParam>,
    Json(payload): Json<PatchReviewRequest>
) -> impl IntoResponse {
    match usecase.execute(UpdateReviewInput {
        id: path.id,
        user_id: auth_user.id(),
        rating: payload.rating as i32,
        content: payload.content,
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), UpdateReviewPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
