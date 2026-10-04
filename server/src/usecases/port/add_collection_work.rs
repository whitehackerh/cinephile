use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::add_collection_work::{
    AddCollectionWorkInput,
    AddCollectionWorkOutput
};

#[async_trait]
pub(crate) trait AddCollectionWorkUseCase: Send + Sync {
    async fn execute(&self, input: AddCollectionWorkInput) -> Result<AddCollectionWorkOutput, AppError>;
}
