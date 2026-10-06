use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::get_collection::{
    GetCollectionInput,
    GetCollectionOutput
};

#[async_trait]
pub(crate) trait GetCollectionUseCase: Send + Sync {
    async fn execute(&self, input: GetCollectionInput) -> Result<GetCollectionOutput, AppError>;
}
