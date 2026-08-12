use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::tv_season::{
            TvSeasonInput,
            TvSeasonOutput,
            EpisodeSummary as EpisodeSummaryDto
        },
        gateway::tmdb::TmdbGateway,
        port::tv_season::TvSeasonUseCase,
    }
};

pub(crate) struct TvSeasonInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl TvSeasonInteractor {
    pub fn new(tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>) -> Self {
        Self { tmdb_gateway }
    }
}

#[async_trait]
impl TvSeasonUseCase for TvSeasonInteractor {
    async fn execute(&self, input: TvSeasonInput) -> Result<TvSeasonOutput, AppError> {
        if input.series_id < 1 {
            return Err(AppError::Validation("Id must be greater than or equal to 1".to_string()));
        }

        let tv_season = self.tmdb_gateway
            .fetch_tv_season(input.series_id, input.season_number)
            .await?;

        let (
            id, season_number, episode_count, title, overview,
            poster_path, air_date, vote_average, episode_summaries
        ) = tv_season.into_parts();

        Ok(TvSeasonOutput {
            id,
            season_number,
            episode_count,
            title,
            overview,
            poster_path,
            air_date,
            vote_average,
            episode_summaries: episode_summaries
                .into_iter()
                .map(|e| {
                    let (
                        e_id, e_episode_number, e_title, e_overview,
                        e_runtime, e_poster_path, e_air_date, e_vote_average
                    ) = e.into_parts();
                    EpisodeSummaryDto {
                        id: e_id, episode_number: e_episode_number, title: e_title,
                        overview: e_overview, runtime: e_runtime, poster_path: e_poster_path,
                        air_date: e_air_date, vote_average: e_vote_average 
                    }
                })
                .collect(),
        })
    }
}