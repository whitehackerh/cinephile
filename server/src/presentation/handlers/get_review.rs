use axum::{
    extract::{Extension, Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::ReviewsPathParam,
    presentation::presenters::{
        base_response::ApiResponse,
        get_review::GetReviewPresenter
    },
    usecases::{
        dto::get_review::GetReviewInput,
        port::get_review::GetReviewUseCase
    }
};

pub async fn get_review_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn GetReviewUseCase + Send + Sync>>,
    Path(path): Path<ReviewsPathParam>
) -> impl IntoResponse {
    match usecase.execute(GetReviewInput {
        id: path.id,
        user_id: auth_user.id(),
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), GetReviewPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
