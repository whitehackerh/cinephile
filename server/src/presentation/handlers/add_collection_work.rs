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
        PostCollectionWorkRequest,
    },
    presentation::presenters::{
        base_response::ApiResponse,
        add_collection_work::AddCollectionWorkPresenter
    },
    usecases::{
        dto::add_collection_work::AddCollectionWorkInput,
        port::add_collection_work::AddCollectionWorkUseCase
    }
};

pub async fn add_collection_work_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn AddCollectionWorkUseCase + Send + Sync>>,
    Path(path): Path<CollectionPathParam>,
    Json(payload): Json<PostCollectionWorkRequest>,
) -> impl IntoResponse {
    match usecase.execute(AddCollectionWorkInput {
        id: path.id,
        user_id: auth_user.id(),
        work_type: payload.work_type.to_string(),
        target_path: payload.target_path
    }).await {
        Ok(output) => {
            (
                StatusCode::CREATED,
                Json(ApiResponse::success(uri.to_string(), AddCollectionWorkPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
