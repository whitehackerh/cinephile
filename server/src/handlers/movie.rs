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
        port::movie::MovieUseCase,
        dto::movie::{
            MovieInput,
            MovieOutput,
            Genre as GenreDto,
        }
    }
};

#[derive(Deserialize)]
pub(crate) struct MoviePath {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MovieResponse {
    pub id: i32,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: Option<f64>,
    pub tagline: Option<String>,
    pub genres: Vec<Genre>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Genre {
    pub id: i32,
    pub name: String,
}

impl From<MovieOutput> for MovieResponse {
    fn from(output: MovieOutput) -> Self {
        Self {
            id: output.id,
            title: output.title,
            original_title: output.original_title,
            overview: output.overview,
            poster_path: output.poster_path,
            backdrop_path: output.backdrop_path,
            release_date: output.release_date,
            runtime: output.runtime,
            vote_average: output.vote_average,
            tagline: output.tagline,
            genres: output.genres.into_iter().map(Genre::from).collect(),
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

pub async fn movie_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn MovieUseCase + Send + Sync>>,
    Path(path): Path<MoviePath>
) -> impl IntoResponse {
    match usecase.execute(MovieInput { id: path.id }).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), MovieResponse::from(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}
