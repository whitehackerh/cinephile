use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    generated::api_schema::TvSeasonPathParams,
    presentation::presenters::{
        base_response::ApiResponse,
        tv_season::TvSeasonPresenter
    },
    usecases::{
        port::tv_season::TvSeasonUseCase,
        dto::tv_season::TvSeasonInput
    }
};

pub async fn tv_season_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvSeasonUseCase + Send + Sync>>,
    Path(path): Path<TvSeasonPathParams>
) -> impl IntoResponse {
    match usecase.execute(TvSeasonInput { series_id: path.series_id.get() as i32, season_number: path.season_number as i32 }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvSeasonPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}