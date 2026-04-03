use uuid::Uuid;

pub(crate) trait TokenManager: Send + Sync {
    fn generate(&self, user_id: Uuid) -> Result<String, String>;
}