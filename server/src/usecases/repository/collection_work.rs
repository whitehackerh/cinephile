use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::entities::collection_work::CollectionWork,
};

#[async_trait]
pub(crate) trait CollectionWorkRepository: Send + Sync {
    async fn create(&self, work: &CollectionWork, collection_id: &Uuid) -> Result<(), anyhow::Error>;
    async fn delete(&self, collection_work_id: &Uuid) -> Result<(), anyhow::Error>;
}
