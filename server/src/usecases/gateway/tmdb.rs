use async_trait::async_trait;
use crate::usecases::dto::search::SearchOutput;
use crate::domain::errors::AppError;

#[async_trait]
pub trait TmdbGateway: Send + Sync {
    async fn fetch_search_results(&self, query: &str, page: i32) -> Result<SearchOutput, AppError>;
}
