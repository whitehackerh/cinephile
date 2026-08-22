use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::{
            genre::Genre,
            tv_series::{
                TvSeriesInput,
                TvSeriesOutput,
                SeasonSummary as SeasonSummaryDto
            },
        },
        gateway::tmdb::TmdbGateway,
        port::tv_series::TvSeriesUseCase,
    }
};

pub(crate) struct TvSeriesInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl TvSeriesInteractor {
    pub fn new(tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>) -> Self {
        Self { tmdb_gateway }
    }
}

#[async_trait]
impl TvSeriesUseCase for TvSeriesInteractor {
    async fn execute(&self, input: TvSeriesInput) -> Result<TvSeriesOutput, AppError> {
        if input.id < 1 {
            return Err(AppError::Validation("Id must be greater than or equal to 1".to_string()));
        }

        let tv_series = self.tmdb_gateway
            .fetch_tv_series_by_id(input.id)
            .await?;

        let (
            id, title, original_title, overview, number_of_seasons, 
            number_of_episodes, poster_path, backdrop_path, first_air_date,
            vote_average, tagline, genres, season_summaries
        ) = tv_series.into_parts();

        Ok(TvSeriesOutput {
            id,
            title,
            original_title,
            overview,
            number_of_seasons,
            number_of_episodes,
            poster_path,
            backdrop_path,
            first_air_date,
            vote_average,
            tagline,
            genres: genres
                .into_iter()
                .map(|g| {
                    let (g_id, g_name) = g.into_parts();
                    Genre { id: g_id, name: g_name }
                })
                .collect(),
            season_summaries: season_summaries
                .into_iter()
                .map(|s| {
                    let (
                        s_id, s_season_number, s_episode_count, s_title,
                        s_overview, s_poster_path, s_air_date, s_vote_average
                    ) = s.into_parts();
                    SeasonSummaryDto {
                        id: s_id, season_number: s_season_number, episode_count: s_episode_count,
                        title: s_title, overview: s_overview, poster_path: s_poster_path,
                        air_date: s_air_date, vote_average: s_vote_average 
                    }
                })
                .collect(),
        })
    }
}