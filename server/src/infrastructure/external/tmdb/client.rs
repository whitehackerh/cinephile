use async_trait::async_trait;
use reqwest::Client;
use crate::{
    domain::{
        entities::{
            genre::Genre,
            movie::Movie,
            tv_episode_summary::TvEpisodeSummary,
            tv_episode::TvEpisode,
            tv_season_summary::TvSeasonSummary,
            tv_season::TvSeason,
            tv_series::TvSeries,
        },
        errors::AppError,
    },
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

use super::types::{TmdbSearchResponse, TmdbMedia, TmdbMovie, TmdbTvEpisode, TmdbTvSeason, TmdbTvSeries};

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
                .expect("Failed to build HTTP client with timeout"),
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
                TmdbMedia::Movie(m) => {
                    let title = m.title?; 
                    Some(Work::Movie(MovieSummary {
                        id: m.id,
                        title,
                        overview: m.overview,
                        poster_path: m.poster_path,
                        release_date: m.release_date,
                    }))
                },
                TmdbMedia::Tv(t) => {
                    let name = t.name?;
                    Some(Work::Tv(TvSummary {
                        id: t.id,
                        name,
                        overview: t.overview,
                        poster_path: t.poster_path,
                        first_air_date: t.first_air_date,
                    }))
                },
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

    async fn fetch_movie_by_id(&self, id: i32) -> Result<Movie, AppError> {
        let response = self.http_client
            .get(format!("{}/movie/{}", self.base_url, id))
            .query(&[
                ("api_key", self.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let response = response.error_for_status().map_err(|e| {
            if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                AppError::EntityNotFound(format!("Movie with id {} not found", id))
            } else {
                AppError::Infrastructure(format!("TMDB API error: {}", e))
            }
        })?;

        let tmdb_res = response
            .json::<TmdbMovie>()
            .await
            .map_err(|e| AppError::Infrastructure(format!("Failed to parse TMDB response: {}", e)))?;

        Ok(
            Movie::new(
                tmdb_res.id,
                tmdb_res.title,
                tmdb_res.original_title,
                tmdb_res.overview,
                tmdb_res.poster_path,
                tmdb_res.backdrop_path,
                tmdb_res.release_date,
                tmdb_res.runtime,
                tmdb_res.vote_average.map(|v| v as f64 * 10.0),
                tmdb_res.tagline,
                tmdb_res.genres
                .unwrap_or_default()
                .into_iter()
                .map(|genre| Genre::new(genre.id, genre.name))
                .collect(),
            )
        )
    }

    async fn fetch_tv_series_by_id(&self, id: i32) -> Result<TvSeries, AppError> {
        let response = self.http_client
            .get(format!("{}/tv/{}", self.base_url, id))
            .query(&[
                ("api_key", self.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let response = response.error_for_status().map_err(|e| {
            if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                AppError::EntityNotFound(format!("TV series with id {} not found", id))
            } else {
                AppError::Infrastructure(format!("TMDB API error: {}", e))
            }
        })?;

        let tmdb_res = response
            .json::<TmdbTvSeries>()
            .await
            .map_err(|e| AppError::Infrastructure(format!("Failed to parse TMDB response: {}", e)))?;

        Ok(
            TvSeries::new(
                tmdb_res.id,
                tmdb_res.name,
                tmdb_res.original_name,
                tmdb_res.overview,
                tmdb_res.number_of_seasons,
                tmdb_res.number_of_episodes,
                tmdb_res.poster_path,
                tmdb_res.backdrop_path,
                tmdb_res.first_air_date,
                tmdb_res.vote_average.map(|v| v as f64 * 10.0),
                tmdb_res.tagline,
                tmdb_res.genres
                .unwrap_or_default()
                .into_iter()
                .map(|genre| Genre::new(genre.id, genre.name))
                .collect(),
                tmdb_res.seasons
                .unwrap_or_default()
                .into_iter()
                .map(|season| TvSeasonSummary::new(
                    season.id, season.season_number, season.episode_count, season.name,
                    season.overview, season.poster_path, season.air_date, season.vote_average.map(|v| v as f64 * 10.0)
                ))
                .collect(),
            )
        )
    }

    async fn fetch_tv_season(&self, series_id: i32, season_number: i32) -> Result<TvSeason, AppError> {
        let response = self.http_client
            .get(format!("{}/tv/{}/season/{}", self.base_url, series_id, season_number))
            .query(&[
                ("api_key", self.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let response = response.error_for_status().map_err(|e| {
            if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                AppError::EntityNotFound(format!("TV season with series_id {} season_number {} not found", series_id, season_number))
            } else {
                AppError::Infrastructure(format!("TMDB API error: {}", e))
            }
        })?;

        let tmdb_res = response
            .json::<TmdbTvSeason>()
            .await
            .map_err(|e| AppError::Infrastructure(format!("Failed to parse TMDB response: {}", e)))?;

        if tmdb_res.season_number != season_number {
            return Err(AppError::Infrastructure(
                "TMDB returned a different season number".to_string(),
            ));
        }

        Ok(
            TvSeason::new(
                tmdb_res.id,
                tmdb_res.season_number,
                tmdb_res
                .episodes
                .as_ref()
                .map_or(0, |episodes| episodes.len() as i32),
                tmdb_res.name,
                tmdb_res.overview,
                tmdb_res.poster_path,
                tmdb_res.air_date,
                tmdb_res.vote_average.map(|v| v as f64 * 10.0),
                tmdb_res.episodes
                .unwrap_or_default()
                .into_iter()
                .map(|episode| TvEpisodeSummary::new(
                    episode.id, episode.episode_number, episode.name, episode.overview,
                    episode.runtime, episode.still_path, episode.air_date, episode.vote_average.map(|v| v as f64 * 10.0)
                ))
                .collect(),
            )
        )
    }

    async fn fetch_tv_episode(&self, series_id: i32, season_number: i32, episode_number: i32) -> Result<TvEpisode, AppError> {
        let response = self.http_client
            .get(format!("{}/tv/{}/season/{}/episode/{}", self.base_url, series_id, season_number, episode_number))
            .query(&[
                ("api_key", self.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let response = response.error_for_status().map_err(|e| {
            if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                AppError::EntityNotFound(format!("TV episode with series_id {} season_number {} episode_number {} not found", series_id, season_number, episode_number))
            } else {
                AppError::Infrastructure(format!("TMDB API error: {}", e))
            }
        })?;

        let tmdb_res = response
            .json::<TmdbTvEpisode>()
            .await
            .map_err(|e| AppError::Infrastructure(format!("Failed to parse TMDB response: {}", e)))?;

        if tmdb_res.season_number != season_number || tmdb_res.episode_number != episode_number {
            return Err(AppError::Infrastructure(
                "TMDB returned a different season number or episode number".to_string(),
            ));
        }

        Ok(
            TvEpisode::new(
                tmdb_res.id,
                tmdb_res.episode_number,
                tmdb_res.season_number,
                tmdb_res.name,
                tmdb_res.overview,
                tmdb_res.runtime,
                tmdb_res.still_path,
                tmdb_res.air_date,
                tmdb_res.vote_average.map(|v| v as f64 * 10.0),
                tmdb_res.production_code,
                tmdb_res.episode_type
            )
        )
    }
}
