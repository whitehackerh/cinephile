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
        port::tv_series::TvSeriesUseCase,
        dto::tv_series::{
            TvSeriesInput,
            TvSeriesOutput,
            Genre as GenreDto,
            SeasonSummary as SeasonSummaryDto,
        }
    }
};

#[derive(Deserialize)]
pub(crate) struct TvSeriesPath {
    series_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSeriesResponse {
    pub id: i32,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub number_of_seasons: Option<i32>,
    pub number_of_episodes: Option<i32>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub tagline: Option<String>,
    pub genres: Vec<Genre>,
    pub season_summaries: Vec<SeasonSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Genre {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SeasonSummary {
    pub id: i32,
    pub season_number: i32,
    pub episode_count: i32,
    pub title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
}

impl From<TvSeriesOutput> for TvSeriesResponse {
    fn from(output: TvSeriesOutput) -> Self {
        Self {
            id: output.id,
            title: output.title,
            original_title: output.original_title,
            overview: output.overview,
            number_of_seasons: output.number_of_seasons,
            number_of_episodes: output.number_of_episodes,
            poster_path: output.poster_path,
            backdrop_path: output.backdrop_path,
            first_air_date: output.first_air_date,
            vote_average: output.vote_average,
            tagline: output.tagline,
            genres: output.genres.into_iter().map(Genre::from).collect(),
            season_summaries: output.season_summaries.into_iter().map(SeasonSummary::from).collect()
        }
    }
}

impl From<GenreDto> for Genre {
    fn from(genre: GenreDto) -> Self {
        Self {
            id: genre.id,
            name: genre.name,
        }
    }
}

impl From<SeasonSummaryDto> for SeasonSummary {
    fn from(output: SeasonSummaryDto)-> Self {
        Self {
            id: output.id,
            season_number: output.season_number,
            episode_count: output.episode_count,
            title: output.title,
            overview: output.overview,
            poster_path: output.poster_path,
            air_date: output.air_date,
            vote_average: output.vote_average,
        }
    }
}

pub async fn tv_series_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn TvSeriesUseCase + Send + Sync>>,
    Path(path): Path<TvSeriesPath>
) -> impl IntoResponse {
    match usecase.execute(TvSeriesInput { id: path.series_id }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), TvSeriesResponse::from(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}