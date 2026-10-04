use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::entities::collection::Collection,
    usecases::dto::collection::{
        CollectionForReconstruct,
        CollectionWithoutWork
    }
};

#[async_trait]
pub(crate) trait CollectionRepository: Send + Sync {
    async fn create(&self, collection: &Collection) -> Result<(), anyhow::Error>;
    async fn find_by_id_with_works(&self, id: &Uuid, user_id: &Uuid) -> anyhow::Result<Option<CollectionForReconstruct>>;
    async fn find_by_id_with_works_for_update(&self, id: &Uuid, user_id: &Uuid) -> anyhow::Result<Option<CollectionForReconstruct>>;
    async fn update(&self, collection: &Collection) -> Result<(), anyhow::Error>;
    async fn fetch_all(&self, user_id: &Uuid) -> anyhow::Result<Vec<CollectionWithoutWork>>;
}
