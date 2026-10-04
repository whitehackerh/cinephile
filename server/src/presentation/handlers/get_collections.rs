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
        get_collections::GetCollectionsPresenter
    },
    usecases::{
        dto::get_collections::GetCollectionsInput,
        port::get_collections::GetCollectionsUseCase
    }
};

pub async fn get_collections_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn GetCollectionsUseCase + Send + Sync>>,
) -> impl IntoResponse {
    match usecase.execute(GetCollectionsInput {
        user_id: auth_user.id(),
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), GetCollectionsPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
