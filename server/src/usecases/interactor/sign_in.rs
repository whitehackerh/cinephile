use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::errors::AppError;
use crate::usecases::dto::sign_in::{SignInInput, SignInOutput};
use crate::usecases::port::sign_in::SignInUseCase;
use crate::usecases::repository::user::UserRepository;
use crate::usecases::security::password::PasswordManager;
use crate::usecases::security::token::TokenManager;

pub(crate) struct SignInInteractor {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    token_manager: Arc<dyn TokenManager>,
    password_manager: Arc<dyn PasswordManager>,
}

impl SignInInteractor {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        token_manager: Arc<dyn TokenManager>,
        password_manager: Arc<dyn PasswordManager>,
    ) -> Self {
        Self { user_repository, token_manager, password_manager }
    } 
}

#[async_trait]
impl SignInUseCase for SignInInteractor {
    async fn execute(&self, input: SignInInput) -> Result<SignInOutput, AppError> {
        let user = self.user_repository
            .find_by_email(&input.email)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

        if !self.password_manager.verify(&input.password, &user.password_hash()) {
            return Err(AppError::Unauthorized("Invalid email or password".into()));
        }

        let token = self.token_manager
            .generate(user.id())
            .map_err(|e| AppError::Infrastructure(e))?;

        Ok(SignInOutput { token })
    }
}