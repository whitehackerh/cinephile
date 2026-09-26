use axum::{
    extract::{Extension, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::PostCollectionsRequest,
    presentation::presenters::{
        base_response::ApiResponse,
        post_collections::PostCollectionsPresenter
    },
    usecases::{
        dto::post_collections::PostCollectionsInput,
        port::post_collections::PostCollectionsUseCase
    }
};

pub async fn post_collections_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn PostCollectionsUseCase + Send + Sync>>,
    Json(payload): Json<PostCollectionsRequest>,
) -> impl IntoResponse {
    match usecase.execute(PostCollectionsInput {
        user_id: auth_user.id(),
        title: payload.title,
        description: payload.description,
    }).await {
        Ok(output) => {
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(uri.to_string(), PostCollectionsPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
