use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::errors::AppError,
    usecases::{
        dto::get_collections::{
            GetCollectionsInput,
            GetCollectionsOutput
        },
        port::get_collections::GetCollectionsUseCase,
        repository::collection::CollectionRepository
    }
};

pub(crate) struct GetCollectionsInteractor {
    collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
}

impl GetCollectionsInteractor {
    pub fn new(
        collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
    ) -> Self {
        Self { collection_repository }
    }
}

#[async_trait]
impl GetCollectionsUseCase for GetCollectionsInteractor {
    async fn execute(&self, input: GetCollectionsInput) -> Result<GetCollectionsOutput, AppError> {
        let collection_without_work_list = self.collection_repository.fetch_all(&input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        Ok(collection_without_work_list)
    }
}
