use axum::{
    extract::{State, Query},
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
        port::search::SearchUseCase,
        dto::search::{
            SearchInput,
            SearchOutput,
            Work as WorkDto
        }
    }
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchResponse {
    pub works: Vec<Work>,
    pub page: u32,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "media_type")]
pub(crate) enum Work {
    #[serde(rename = "movie")]
    Movie(MovieSummary),
    #[serde(rename = "tv")]
    Tv(TvSummary),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MovieSummary {
    pub id: i32,
    pub title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSummary {
    pub id: i32,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
}

impl From<SearchOutput> for SearchResponse {
    fn from(output: SearchOutput) -> Self {
        Self {
            works: output.works.into_iter().map(Work::from).collect(),
            page: output.page,
            total_pages: output.total_pages,
            total_results: output.total_results,
        }
    }
}

impl From<WorkDto> for Work {
    fn from(work: WorkDto) -> Self {
        match work {
            WorkDto::Movie(m) => Work::Movie(MovieSummary {
                id: m.id,
                title: m.title,
                overview: m.overview,
                poster_path: m.poster_path,
                release_date: m.release_date,
            }),
            WorkDto::Tv(t) => Work::Tv(TvSummary {
                id: t.id,
                name: t.name,
                overview: t.overview,
                poster_path: t.poster_path,
                first_air_date: t.first_air_date,
            })
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct SearchParams {
    q: String,
    page: Option<u32>,
}

pub async fn search_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn SearchUseCase + Send + Sync>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let input = SearchInput {
        query: params.q,
        page: params.page.unwrap_or(1),
    };

    match usecase.execute(input).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), SearchResponse::from(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
    }
}
