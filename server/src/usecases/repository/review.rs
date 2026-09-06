use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::entities::review::Review,
    usecases::dto::review::ReviewWithoutWork
};

#[async_trait]
pub(crate) trait ReviewRepository: Send + Sync {
    async fn exists_by_user_and_target(&self, user_id: &Uuid, target_path: &str) -> anyhow::Result<bool>;
    async fn create(&self, review: &Review) -> Result<(), anyhow::Error>;
    async fn find_by_work_type_target_path(&self, user_id: &Uuid, work_type: &str, target_path: &str) -> anyhow::Result<Option<ReviewWithoutWork>>;
}
