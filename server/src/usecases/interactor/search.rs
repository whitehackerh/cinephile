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
        if input.query.trim().is_empty() {
            return Err(AppError::Validation("Query cannot be empty".to_string()));
        }

        let output = self.tmdb_gateway
            .fetch_search_results(&input.query, input.page)
            .await?;

        Ok(output)
    }
}
