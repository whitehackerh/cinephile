use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    generated::api_schema::TvEpisodePathParams,
    presentation::presenters::{
        base_response::ApiResponse,
        tv_episode::TvEpisodePresenter
    },
    usecases::{
        port::tv_episode::TvEpisodeUseCase,
        dto::tv_episode::TvEpisodeInput
    }
};

pub async fn tv_episode_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvEpisodeUseCase + Send + Sync>>,
    Path(path): Path<TvEpisodePathParams>
) -> impl IntoResponse {
    match usecase.execute(TvEpisodeInput { 
        series_id: path.series_id.get()as i32,
        season_number: path.season_number as i32, 
        episode_number: path.episode_number as i32 
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvEpisodePresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}