use async_trait::async_trait;
use crate::{
    domain::entities::collection::Collection,
};

#[async_trait]
pub(crate) trait CollectionRepository: Send + Sync {
    async fn create(&self, collection: &Collection) -> Result<(), anyhow::Error>;
}
