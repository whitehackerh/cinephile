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
        create_collection::CreateCollectionPresenter
    },
    usecases::{
        dto::create_collection::CreateCollectionInput,
        port::create_collection::CreateCollectionUseCase
    }
};

pub async fn create_collection_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn CreateCollectionUseCase + Send + Sync>>,
    Json(payload): Json<PostCollectionsRequest>,
) -> impl IntoResponse {
    match usecase.execute(CreateCollectionInput {
        user_id: auth_user.id(),
        title: payload.title,
        description: payload.description,
    }).await {
        Ok(output) => {
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(uri.to_string(), CreateCollectionPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
