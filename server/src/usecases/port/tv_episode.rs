use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::tv_episode::{TvEpisodeInput, TvEpisodeOutput};

#[async_trait]
pub(crate) trait TvEpisodeUseCase: Send + Sync {
    async fn execute(&self, input: TvEpisodeInput) -> Result<TvEpisodeOutput, AppError>;
}