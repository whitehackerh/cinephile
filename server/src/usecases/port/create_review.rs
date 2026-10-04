use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::create_review::{
    CreateReviewInput,
    CreateReviewOutput
};

#[async_trait]
pub(crate) trait CreateReviewUseCase: Send + Sync {
    async fn execute(&self, input: CreateReviewInput) -> Result<CreateReviewOutput, AppError>;
}
