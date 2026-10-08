use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::delete_collection::DeleteCollectionInput;

#[async_trait]
pub(crate) trait DeleteCollectionUseCase: Send + Sync {
    async fn execute(&self, input: DeleteCollectionInput) -> Result<(), AppError>;
}
