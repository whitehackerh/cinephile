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
        CollectionPathParam,
        PatchCollectionRequest,
    },
    presentation::presenters::{
        base_response::ApiResponse,
        update_collection::UpdateCollectionPresenter
    },
    usecases::{
        dto::update_collection::UpdateCollectionInput,
        port::update_collection::UpdateCollectionUseCase
    }
};

pub async fn update_collection_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn UpdateCollectionUseCase + Send + Sync>>,
    Path(path): Path<CollectionPathParam>,
    Json(payload): Json<PatchCollectionRequest>,
) -> impl IntoResponse {
    match usecase.execute(UpdateCollectionInput {
        id: path.id,
        user_id: auth_user.id(),
        title: payload.title,
        description: payload.description,
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), UpdateCollectionPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
