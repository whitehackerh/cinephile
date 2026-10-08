use axum::{
    extract::{Extension, Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::json;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::ReviewPathParam,
    presentation::presenters::base_response::ApiResponse,
    usecases::{
        dto::delete_review::DeleteReviewInput,
        port::delete_review::DeleteReviewUseCase
    }
};

pub async fn delete_review_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn DeleteReviewUseCase + Send + Sync>>,
    Path(path): Path<ReviewPathParam>,
) -> impl IntoResponse {
    match usecase.execute(DeleteReviewInput {
        id: path.id,
        user_id: auth_user.id()
    }).await {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::success(uri.to_string(), json!({})))).into_response(),
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
