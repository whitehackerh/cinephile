use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::AppError;
use crate::domain::entities::user::User;
use crate::infrastructure::security::password::PasswordManager;
use crate::usecases::dto::SignUpInput;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::usecases::repository::user::UserRepository;

pub struct SignUpInteractor {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl SignUpInteractor {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl SignUpUseCase for SignUpInteractor {
    async fn execute(&self, input: SignUpInput) -> Result<(), AppError> {
        if let Some(_) = self.user_repository
            .find_by_email(&input.email)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))? 
        {
            return Err(AppError::AlreadyExists("Email already taken".into()));
        }

        let hashed_password = PasswordManager::hash(&input.password)
            .map_err(|e| AppError::Infrastructure(e.into()))?;

        let user = User::new(
            Uuid::new_v4(),
            input.name,
            input.email,
            hashed_password,
        )?;

        self.user_repository.create(&user).await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        Ok(())
    }
}
