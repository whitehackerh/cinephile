use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::AppError;
use crate::domain::entities::user::User;
use crate::usecases::dto::sign_up::SignUpInput;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::usecases::repository::user::UserRepository;
use crate::usecases::security::password::PasswordManager;

pub struct SignUpInteractor {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    password_manager: Arc<dyn PasswordManager>,
}

impl SignUpInteractor {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        password_manager: Arc<dyn PasswordManager>,
    ) -> Self {
        Self { user_repository, password_manager }
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

        let hashed_password =self.password_manager.hash(&input.password)
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
