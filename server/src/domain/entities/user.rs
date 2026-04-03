use uuid::Uuid;
use crate::domain::errors::AppError;

#[derive(Debug, Clone)]
pub(crate) struct User {
    id: Uuid,
    name: String,
    email: String,
    password_hash: String,
}

impl User {
    pub fn new(id: Uuid, name: String, email: String, password_hash: String) -> Result<Self, AppError> {
        if name.is_empty() {
            return Err(AppError::Validation("Name cannot be empty".into()));
        }
        if name.chars().count() > 50 {
            return Err(AppError::Validation("User name must be 50 characters or less".into()));
        }
        if !email.contains('@') {
            return Err(AppError::Validation("Invalid email format".into()));
        }

        Ok(Self { id, name, email, password_hash })
    }

    pub fn reconstruct(id: Uuid, name: String, email: String, password_hash: String) -> Self {
        Self { id, name, email, password_hash }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}
