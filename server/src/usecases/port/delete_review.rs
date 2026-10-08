use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::delete_review::DeleteReviewInput;

#[async_trait]
pub(crate) trait DeleteReviewUseCase: Send + Sync {
    async fn execute(&self, input: DeleteReviewInput) -> Result<(), AppError>;
}
