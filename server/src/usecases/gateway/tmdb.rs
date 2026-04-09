use async_trait::async_trait;
use crate::usecases::dto::search::SearchOutput;
use crate::domain::{
    entities::movie::Movie,
    errors::AppError
};

#[async_trait]
pub(crate) trait TmdbGateway: Send + Sync {
    async fn fetch_search_results(&self, query: &str, page: u32) -> Result<SearchOutput, AppError>;
    async fn fetch_movie_by_id(&self, id: i32) -> Result<Movie, AppError>;
}
