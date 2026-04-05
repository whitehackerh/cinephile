use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct AuthUser {
    id: Uuid,
}

impl AuthUser {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
