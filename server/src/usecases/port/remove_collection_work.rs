use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::remove_collection_work::RemoveCollectionWorkInput;

#[async_trait]
pub(crate) trait RemoveCollectionWorkUseCase: Send + Sync {
    async fn execute(&self, input: RemoveCollectionWorkInput) -> Result<(), AppError>;
}
