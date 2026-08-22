use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::post_reviews::{
    PostReviewsInput,
    PostReviewsOutput
};

#[async_trait]
pub(crate) trait PostReviewsUseCase: Send + Sync {
    async fn execute(&self, input: PostReviewsInput) -> Result<PostReviewsOutput, AppError>;
}
