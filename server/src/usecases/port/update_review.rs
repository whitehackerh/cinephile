use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::update_review::{
    UpdateReviewInput,
    UpdateReviewOutput
};

#[async_trait]
pub(crate) trait UpdateReviewUseCase: Send + Sync {
    async fn execute(&self, input: UpdateReviewInput) -> Result<UpdateReviewOutput, AppError>;
}
