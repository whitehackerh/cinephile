use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::sign_up::SignUpInput;

#[async_trait]
pub(crate) trait SignUpUseCase: Send + Sync {
    async fn execute(&self, input: SignUpInput) -> Result<(), AppError>;
}
