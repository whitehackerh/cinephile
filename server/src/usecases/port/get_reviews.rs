use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::get_reviews::{
    GetReviewsInput,
    GetReviewsOutput
};

#[async_trait]
pub(crate) trait GetReviewsUseCase: Send + Sync {
    async fn execute(&self, input: GetReviewsInput) -> Result<GetReviewsOutput, AppError>;
}
