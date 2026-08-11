use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::tv_series::{TvSeriesInput, TvSeriesOutput};

#[async_trait]
pub(crate) trait TvSeriesUseCase: Send + Sync {
    async fn execute(&self, input: TvSeriesInput) -> Result<TvSeriesOutput, AppError>;
}