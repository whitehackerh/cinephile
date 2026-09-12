use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::delete_reviews::DeleteReviewsInput;

#[async_trait]
pub(crate) trait DeleteReviewsUseCase: Send + Sync {
    async fn execute(&self, input: DeleteReviewsInput) -> Result<(), AppError>;
}
