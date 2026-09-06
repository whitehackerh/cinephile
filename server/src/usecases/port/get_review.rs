use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::get_review::{
    GetReviewInput,
    GetReviewOutput
};

#[async_trait]
pub(crate) trait GetReviewUseCase: Send + Sync {
    async fn execute(&self, input: GetReviewInput) -> Result<GetReviewOutput, AppError>;
}
