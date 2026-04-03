use async_trait::async_trait;
use crate::domain::entities::user::User;

#[async_trait]
pub(crate) trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
    async fn create(&self, user: &User) -> Result<(), anyhow::Error>;
}
