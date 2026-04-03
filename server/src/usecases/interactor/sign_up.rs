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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use crate::domain::entities::user::User;
    use crate::usecases::repository::user::UserRepository;
    use crate::usecases::security::password::PasswordManager;

    // --- Mock: UserRepository ---
    struct MockUserRepository {
        existing_user: Option<User>,
        find_fail: bool,
        create_fail: bool,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, anyhow::Error> {
            if self.find_fail {
                Err(anyhow::anyhow!("db find error"))
            } else {
                Ok(self.existing_user.clone())
            }
        }

        async fn create(&self, _user: &User) -> Result<(), anyhow::Error> {
            if self.create_fail {
                Err(anyhow::anyhow!("db create error"))
            } else {
                Ok(())
            }
        }
    }

    // --- Mock: PasswordManager ---
    struct MockPasswordManager {
        hash_result: Result<String, String>,
    }

    impl PasswordManager for MockPasswordManager {
        fn hash(&self, _password: &str) -> Result<String, String> {
            self.hash_result.clone()
        }

        fn verify(&self, _password: &str, _hash: &str) -> bool {
            true
        }
    }

    fn make_interactor(
        existing_user: Option<User>,
        find_fail: bool,
        create_fail: bool,
        hash_result: Result<String, String>,
    ) -> SignUpInteractor {
        SignUpInteractor::new(
            Arc::new(MockUserRepository { existing_user, find_fail, create_fail }),
            Arc::new(MockPasswordManager { hash_result }),
        )
    }

    fn make_input(name: &str, email: &str, password: &str) -> SignUpInput {
        SignUpInput {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    fn make_existing_user() -> User {
        User::reconstruct(
            uuid::Uuid::new_v4(),
            "Existing User".to_string(),
            "existing@example.com".to_string(),
            "some_hash".to_string(),
        )
    }

    #[tokio::test]
    async fn execute_succeeds_for_new_user() {
        let interactor = make_interactor(None, false, false, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input("Alice", "alice@example.com", "secret"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn execute_returns_already_exists_for_duplicate_email() {
        let existing = make_existing_user();
        let interactor = make_interactor(Some(existing), false, false, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input("Bob", "existing@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::AlreadyExists(msg) => assert_eq!(msg, "Email already taken"),
            other => panic!("Expected AlreadyExists, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_infrastructure_error_when_find_fails() {
        let interactor = make_interactor(None, true, false, Ok("hash".to_string()));
        let result = interactor
            .execute(make_input("Alice", "alice@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Infrastructure(_) => {}
            other => panic!("Expected Infrastructure, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_infrastructure_error_when_hash_fails() {
        let interactor = make_interactor(
            None,
            false,
            false,
            Err("hashing error".to_string()),
        );
        let result = interactor
            .execute(make_input("Alice", "alice@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Infrastructure(msg) => assert!(msg.contains("hashing error")),
            other => panic!("Expected Infrastructure, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_infrastructure_error_when_create_fails() {
        let interactor = make_interactor(None, false, true, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input("Alice", "alice@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Infrastructure(_) => {}
            other => panic!("Expected Infrastructure, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_validation_error_for_empty_name() {
        let interactor = make_interactor(None, false, false, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input("", "alice@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("Name cannot be empty")),
            other => panic!("Expected Validation, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_validation_error_for_invalid_email() {
        let interactor = make_interactor(None, false, false, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input("Alice", "not-an-email", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("Invalid email format")),
            other => panic!("Expected Validation, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn execute_returns_validation_error_for_name_too_long() {
        let long_name = "A".repeat(51);
        let interactor = make_interactor(None, false, false, Ok("hashed_pw".to_string()));
        let result = interactor
            .execute(make_input(&long_name, "alice@example.com", "pass"))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert!(msg.contains("50 characters")),
            other => panic!("Expected Validation, got {:?}", other),
        }
    }
}