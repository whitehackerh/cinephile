use async_trait::async_trait;
use reqwest::Client;
use crate::{
    domain::errors::AppError,
    usecases::{
        dto::search::{
            SearchOutput,
            Work,
            MovieSummary,
            TvSummary,
        },
        gateway::tmdb::TmdbGateway,
    }
};

use super::types::{TmdbSearchResponse, TmdbMedia};

pub struct TmdbClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl TmdbClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            api_key,
            base_url,
        }
    }
}

#[async_trait]
impl TmdbGateway for TmdbClient {
    async fn fetch_search_results(&self, query: &str, page: u32) -> Result<SearchOutput, AppError> {
        let response = self.http_client
            .get(format!("{}/search/multi", self.base_url))
            .query(&[
                ("api_key", self.api_key.as_str()),
                ("query", query),
                ("page", &page.to_string()),
            ])
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let tmdb_res = response
            .json::<TmdbSearchResponse>()
            .await
            .map_err(|e| AppError::Infrastructure(format!("Failed to parse TMDB response: {}", e)))?;

        let works = tmdb_res.results.into_iter().filter_map(|media| {
            match media {
                TmdbMedia::Movie(m) => Some(Work::Movie(MovieSummary {
                    id: m.id,
                    title: m.title,
                    overview: m.overview,
                    poster_path: m.poster_path,
                    release_date: m.release_date,
                })),
                TmdbMedia::Tv(t) => Some(Work::Tv(TvSummary {
                    id: t.id,
                    name: t.name,
                    overview: t.overview,
                    poster_path: t.poster_path,
                    first_air_date: t.first_air_date,
                })),
                TmdbMedia::Unknown => None,
            }
        }).collect();

        Ok(SearchOutput {
            works,
            page: tmdb_res.page,
            total_pages: tmdb_res.total_pages,
            total_results: tmdb_res.total_results,
        })
    }
}
