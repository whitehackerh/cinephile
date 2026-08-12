use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::tv_season::{TvSeasonInput, TvSeasonOutput};

#[async_trait]
pub(crate) trait TvSeasonUseCase: Send + Sync {
    async fn execute(&self, input: TvSeasonInput) -> Result<TvSeasonOutput, AppError>;
}