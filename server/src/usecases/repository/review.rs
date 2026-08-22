use async_trait::async_trait;
use crate::domain::entities::review::Review;

#[async_trait]
pub(crate) trait ReviewRepository: Send + Sync {
    async fn create(&self, review: &Review) -> Result<(), anyhow::Error>;
}
