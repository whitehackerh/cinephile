use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::sign_in::{SignInInput, SignInOutput};

#[async_trait]
pub(crate) trait SignInUseCase: Send + Sync {
    async fn execute(&self, input: SignInInput) -> Result<SignInOutput, AppError>;
}