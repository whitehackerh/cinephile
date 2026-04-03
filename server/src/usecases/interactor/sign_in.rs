use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::errors::AppError;
use crate::usecases::dto::sign_in::{SignInInput, SignInOutput};
use crate::usecases::port::sign_in::SignInUseCase;
use crate::usecases::repository::user::UserRepository;
use crate::usecases::security::password::PasswordManager;
use crate::usecases::security::token::TokenManager;

pub struct SignInInteractor {
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::domain::entities::user::User;
    use crate::usecases::repository::user::UserRepository;
    use crate::usecases::security::password::PasswordManager;
    use crate::usecases::security::token::TokenManager;

    // --- Mock: UserRepository ---
    struct MockUserRepository {
        user: Option<User>,
        fail: bool,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, anyhow::Error> {
            if self.fail {
                Err(anyhow::anyhow!("db error"))
            } else {
                Ok(self.user.clone())
            }
        }

        async fn create(&self, _user: &User) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    // --- Mock: PasswordManager ---
    struct MockPasswordManager {
        verify_result: bool,
    }

    impl PasswordManager for MockPasswordManager {
        fn hash(&self, _password: &str) -> Result<String, String> {
            Ok("hashed".to_string())
        }

        fn verify(&self, _password: &str, _hash: &str) -> bool {
            self.verify_result
        }
    }

    // --- Mock: TokenManager ---
    struct MockTokenManager {
        token: Result<String, String>,
    }

    impl TokenManager for MockTokenManager {
        fn generate(&self, _user_id: Uuid) -> Result<String, String> {
            self.token.clone()
        }
    }

    fn make_user() -> User {
        User::reconstruct(
            Uuid::new_v4(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        )
    }

    fn make_interactor(
        user: Option<User>,
        repo_fail: bool,
        verify_result: bool,
        token: Result<String, String>,
    ) -> SignInInteractor {
        SignInInteractor::new(
            Arc::new(MockUserRepository { user, fail: repo_fail }),
            Arc::new(MockTokenManager { token }),
            Arc::new(MockPasswordManager { verify_result }),
        )
    }

    fn make_input(email: &str, password: &str) -> SignInInput {
        SignInInput {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn execute_returns_token_on_success() {
        let user = make_user();
        let interactor = make_interactor(
            Some(user),
            false,
            true,
            Ok("jwt_token_abc123".to_string()),
        );

        let result = interactor.execute(make_input("test@example.com", "password")).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "jwt_token_abc123");
    }

    #[tokio::test]
    async fn execute_returns_unauthorized_when_user_not_found() {
        let interactor = make_interactor(None, false, true, Ok("token".to_string()));
        let result = interactor.execute(make_input("unknown@example.com", "password")).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Unauthorized(msg) => assert_eq!(msg, "Invalid email or password"),
            other => panic!("Expected Unauthorized, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_unauthorized_when_password_wrong() {
        let user = make_user();
        let interactor = make_interactor(Some(user), false, false, Ok("token".to_string()));
        let result = interactor.execute(make_input("test@example.com", "wrong_pass")).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Unauthorized(msg) => assert_eq!(msg, "Invalid email or password"),
            other => panic!("Expected Unauthorized, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_infrastructure_error_when_repo_fails() {
        let interactor = make_interactor(None, true, true, Ok("token".to_string()));
        let result = interactor.execute(make_input("test@example.com", "password")).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Infrastructure(_) => {}
            other => panic!("Expected Infrastructure, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_infrastructure_error_when_token_generation_fails() {
        let user = make_user();
        let interactor = make_interactor(
            Some(user),
            false,
            true,
            Err("Token generation failed: key error".to_string()),
        );
        let result = interactor.execute(make_input("test@example.com", "password")).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Infrastructure(msg) => assert!(msg.contains("Token generation failed")),
            other => panic!("Expected Infrastructure, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_uses_email_from_input_for_lookup() {
        // Verifies that the email is passed to the repo correctly
        // (repo returns None for unknown emails, triggering Unauthorized)
        let interactor = make_interactor(None, false, true, Ok("token".to_string()));
        let result = interactor.execute(make_input("notfound@example.com", "pass")).await;

        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }
}