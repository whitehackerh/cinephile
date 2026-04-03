use uuid::Uuid;

pub trait TokenManager: Send + Sync {
    fn generate(&self, user_id: Uuid) -> Result<String, String>;
}