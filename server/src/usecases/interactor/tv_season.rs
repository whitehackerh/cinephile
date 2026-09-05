use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::tv_season::{
            TvSeasonInput,
            TvSeasonOutput
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
        if input.season_number < 0 {
            return Err(AppError::Validation(
                "Season number must be greater than or equal to 0".to_string(),
            ));
        }

        let tv_season = self.tmdb_gateway
            .fetch_tv_season(input.series_id, input.season_number)
            .await?;

        Ok(TvSeasonOutput::from(tv_season))
    }
}