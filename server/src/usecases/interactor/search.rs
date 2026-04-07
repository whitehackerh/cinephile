use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::errors::AppError,
    usecases::{
        dto::search::{
            SearchInput,
            SearchOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::search::SearchUseCase,
    }
};

pub(crate) struct SearchInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl SearchInteractor {
    pub fn new(tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>) -> Self {
        Self { tmdb_gateway }
    }
}

#[async_trait]
impl SearchUseCase for SearchInteractor {
    async fn execute(&self, input: SearchInput) -> Result<SearchOutput, AppError> {
        let query = input.query.trim();
        if query.is_empty() {
            return Err(AppError::Validation("Query cannot be empty".to_string()));
        }
        if input.page < 1 {
            return Err(AppError::Validation("Page must be greater than or equal to 1".to_string()));
        }

        let output = self.tmdb_gateway
            .fetch_search_results(query, input.page)
            .await?;

        Ok(output)
    }
}
