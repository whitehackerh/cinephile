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
    generated::api_schema::CollectionWorkPathParams,
    presentation::presenters::base_response::ApiResponse,
    usecases::{
        dto::remove_collection_work::RemoveCollectionWorkInput,
        port::remove_collection_work::RemoveCollectionWorkUseCase
    }
};

pub async fn remove_collection_work_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn RemoveCollectionWorkUseCase + Send + Sync>>,
    Path(path): Path<CollectionWorkPathParams>,
) -> impl IntoResponse {
    match usecase.execute(RemoveCollectionWorkInput {
        id: path.id,
        collection_work_id: path.collection_work_id,
        user_id: auth_user.id()
    }).await {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::success(uri.to_string(), json!({})))).into_response(),
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
