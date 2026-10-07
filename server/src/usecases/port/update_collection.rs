use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::update_collection::{
    UpdateCollectionInput,
    UpdateCollectionOutput
};

#[async_trait]
pub(crate) trait UpdateCollectionUseCase: Send + Sync {
    async fn execute(&self, input: UpdateCollectionInput) -> Result<UpdateCollectionOutput, AppError>;
}
