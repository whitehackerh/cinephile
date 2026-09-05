use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::{
            tv_series::{
                TvSeriesInput,
                TvSeriesOutput
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

        Ok(TvSeriesOutput::from(tv_series))
    }
}