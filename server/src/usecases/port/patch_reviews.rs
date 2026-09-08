use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::patch_reviews::{
    PatchReviewsInput,
    PatchReviewsOutput
};

#[async_trait]
pub(crate) trait PatchReviewsUseCase: Send + Sync {
    async fn execute(&self, input: PatchReviewsInput) -> Result<PatchReviewsOutput, AppError>;
}
