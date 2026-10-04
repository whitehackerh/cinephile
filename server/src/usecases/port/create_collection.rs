use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::create_collection::{
    CreateCollectionInput,
    CreateCollectionOutput
};

#[async_trait]
pub(crate) trait CreateCollectionUseCase: Send + Sync {
    async fn execute(&self, input: CreateCollectionInput) -> Result<CreateCollectionOutput, AppError>;
}
