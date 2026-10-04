use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::get_collections::{
    GetCollectionsInput,
    GetCollectionsOutput
};

#[async_trait]
pub(crate) trait GetCollectionsUseCase: Send + Sync {
    async fn execute(&self, input: GetCollectionsInput) -> Result<GetCollectionsOutput, AppError>;
}
