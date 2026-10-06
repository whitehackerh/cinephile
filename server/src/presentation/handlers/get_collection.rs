use axum::{
    extract::{Extension, Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    domain::entities::auth_user::AuthUser,
    generated::api_schema::CollectionPathParam,
    presentation::presenters::{
        base_response::ApiResponse,
        get_collection::GetCollectionPresenter
    },
    usecases::{
        dto::get_collection::GetCollectionInput,
        port::get_collection::GetCollectionUseCase
    }
};

pub async fn get_collection_handler(
    uri: Uri,
    Extension(auth_user): Extension<AuthUser>,
    State(usecase): State<Arc<dyn GetCollectionUseCase + Send + Sync>>,
    Path(path): Path<CollectionPathParam>,
) -> impl IntoResponse {
    match usecase.execute(GetCollectionInput {
        id: path.id,
        user_id: auth_user.id(),
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), GetCollectionPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
