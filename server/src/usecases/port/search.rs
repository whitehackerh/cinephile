use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::search::{SearchInput, SearchOutput};

#[async_trait]
pub(crate) trait SearchUseCase: Send + Sync {
    async fn execute(&self, input: SearchInput) -> Result<SearchOutput, AppError>;
}
