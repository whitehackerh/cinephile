use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::post_collections::{
    PostCollectionsInput,
    PostCollectionsOutput
};

#[async_trait]
pub(crate) trait PostCollectionsUseCase: Send + Sync {
    async fn execute(&self, input: PostCollectionsInput) -> Result<PostCollectionsOutput, AppError>;
}
