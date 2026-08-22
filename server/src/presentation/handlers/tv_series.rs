use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    generated::api_schema::TvSeriesPathParams,
    presentation::presenters::{
        base_response::ApiResponse,
        tv_series::TvSeriesPresenter
    },
    usecases::{
        port::tv_series::TvSeriesUseCase,
        dto::tv_series:: TvSeriesInput
    }
};

pub async fn tv_series_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvSeriesUseCase + Send + Sync>>,
    Path(path): Path<TvSeriesPathParams>
) -> impl IntoResponse {
    match usecase.execute(TvSeriesInput { id: path.series_id.get() as i32 }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvSeriesPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}