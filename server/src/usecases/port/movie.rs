use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::movie::{MovieInput, MovieOutput};

#[async_trait]
pub(crate) trait MovieUseCase: Send + Sync {
    async fn execute(&self, input: MovieInput) -> Result<MovieOutput, AppError>;
}
