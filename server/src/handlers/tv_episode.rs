use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::{
    handlers::base_response::ApiResponse,
    usecases::{
        port::tv_episode::TvEpisodeUseCase,
        dto::tv_episode::{
            TvEpisodeInput,
            TvEpisodeOutput
        }
    }
};

#[derive(Deserialize)]
pub(crate) struct TvEpisodePath {
    series_id: i32,
    season_number: i32,
    episode_number: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvEpisodeResponse {
    pub id: i32,
    pub episode_number: i32,
    pub season_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub production_code: Option<String>,
    pub episode_type: Option<String>
}

impl From<TvEpisodeOutput> for TvEpisodeResponse {
    fn from(output: TvEpisodeOutput) -> Self {
        Self {
            id: output.id,
            episode_number: output.episode_number,
            season_number: output.season_number,
            title: output.title,
            overview: output.overview,
            runtime: output.runtime,
            still_path: output.still_path,
            air_date: output.air_date,
            vote_average: output.vote_average,
            production_code: output.production_code,
            episode_type: output.episode_type
        }
    }
}

pub async fn tv_episode_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvEpisodeUseCase + Send + Sync>>,
    Path(path): Path<TvEpisodePath>
) -> impl IntoResponse {
    match usecase.execute(TvEpisodeInput { 
        series_id: path.series_id,
        season_number: path.season_number, 
        episode_number: path.episode_number 
    }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvEpisodeResponse::from(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}