use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct AuthUser {
    id: Uuid,
    token: String,
}

impl AuthUser {
    pub fn new(id: Uuid, token: String) -> Self {
        Self { id, token }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
