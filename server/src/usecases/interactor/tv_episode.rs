use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::tv_episode::{
            TvEpisodeInput,
            TvEpisodeOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::tv_episode::TvEpisodeUseCase,
    }
};

pub(crate) struct TvEpisodeInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl TvEpisodeInteractor {
    pub fn new(tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>) -> Self {
        Self { tmdb_gateway }
    }
}

#[async_trait]
impl TvEpisodeUseCase for TvEpisodeInteractor {
    async fn execute(&self, input: TvEpisodeInput) -> Result<TvEpisodeOutput, AppError> {
        if input.series_id < 1 {
            return Err(AppError::Validation("Id must be greater than or equal to 1".to_string()));
        }
        if input.season_number < 0 {
            return Err(AppError::Validation(
                "Season number must be greater than or equal to 0".to_string(),
            ));
        }
        if input.episode_number < 0 {
            return Err(AppError::Validation(
                "Episode number must be greater than or equal to 0".to_string(),
            ));
        }

        let tv_episode = self.tmdb_gateway
            .fetch_tv_episode(input.series_id, input.season_number, input.episode_number)
            .await?;

        Ok(TvEpisodeOutput::from(tv_episode))
    }
}