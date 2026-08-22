use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    generated::api_schema::MoviePathParams,
    presentation::presenters::{
        base_response::ApiResponse,
        movie::MoviePresenter
    },
    usecases::{
        port::movie::MovieUseCase,
        dto::movie::MovieInput
    }
};

pub async fn movie_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn MovieUseCase + Send + Sync>>,
    Path(path): Path<MoviePathParams>
) -> impl IntoResponse {
    match usecase.execute(MovieInput { id: path.id.get() as i32 }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), MoviePresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
