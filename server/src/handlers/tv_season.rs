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
        port::tv_season::TvSeasonUseCase,
        dto::tv_season::{
            TvSeasonInput,
            TvSeasonOutput,
            EpisodeSummary as EpisodeSummaryDto,
        }
    }
};

#[derive(Deserialize)]
pub(crate) struct TvSeasonPath {
    series_id: i32,
    season_number: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSeasonResponse {
    pub id: i32,
    pub season_number: i32,
    pub episode_count: i32,
    pub title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub episode_summaries: Vec<EpisodeSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EpisodeSummary {
    pub id: i32,
    pub episode_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
}

impl From<TvSeasonOutput> for TvSeasonResponse {
    fn from(output: TvSeasonOutput) -> Self {
        Self {
            id: output.id,
            season_number: output.season_number,
            episode_count: output.episode_count,
            title: output.title,
            overview: output.overview,
            poster_path: output.poster_path,
            air_date: output.air_date,
            vote_average: output.vote_average,
            episode_summaries: output.episode_summaries.into_iter().map(EpisodeSummary::from).collect()
        }
    }
}

impl From<EpisodeSummaryDto> for EpisodeSummary {
    fn from(output: EpisodeSummaryDto)-> Self {
        Self {
            id: output.id,
            episode_number: output.episode_number,
            title: output.title,
            overview: output.overview,
            runtime: output.runtime,
            still_path: output.still_path,
            air_date: output.air_date,
            vote_average: output.vote_average,
        }
    }
}

pub async fn tv_season_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvSeasonUseCase + Send + Sync>>,
    Path(path): Path<TvSeasonPath>
) -> impl IntoResponse {
    match usecase.execute(TvSeasonInput { series_id: path.series_id, season_number: path.season_number }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvSeasonResponse::from(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}