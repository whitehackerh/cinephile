use async_trait::async_trait;
use crate::domain::errors::AppError;
use crate::usecases::dto::SignUpInput;

#[async_trait]
pub trait SignUpUseCase: Send + Sync {
    async fn execute(&self, input: SignUpInput) -> Result<(), AppError>;
}
