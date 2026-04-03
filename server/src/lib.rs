pub(crate) mod domain;
pub(crate) mod usecases;
pub(crate) mod handlers;
pub mod infrastructure;

use std::sync::Arc;
use sqlx::PgPool;
use crate::infrastructure::persistence::postgres::user::PostgresUserRepository;
use crate::infrastructure::security::password::PasswordManager;
use crate::infrastructure::security::token::JwtTokenManager;
use crate::usecases::interactor::sign_up::SignUpInteractor;
use crate::usecases::interactor::sign_in::SignInInteractor;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::usecases::port::sign_in::SignInUseCase;
use crate::usecases::repository::user::UserRepository;
use crate::usecases::security::password::PasswordManager as PasswordManagerTrait;
use crate::usecases::security::token::TokenManager;

pub struct AppRegistry {
    pub(crate) signup_usecase: Arc<dyn SignUpUseCase + Send + Sync>,
    pub(crate) signin_usecase: Arc<dyn SignInUseCase + Send + Sync>,
}

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<AppRegistry>);

impl AppRegistry {
    pub async fn build(pool: PgPool) -> Arc<Self> {
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");

        let user_repository = Arc::new(PostgresUserRepository::new(pool));
        let password_manager = Arc::new(PasswordManager::new());
        let token_manager = Arc::new(JwtTokenManager::new(jwt_secret));

        let signup_usecase = Arc::new(SignUpInteractor::new(
            user_repository.clone() as Arc<dyn UserRepository + Send + Sync>,
            password_manager.clone() as Arc<dyn PasswordManagerTrait>,
        ));
        let signin_usecase = Arc::new(SignInInteractor::new(
            user_repository.clone() as Arc<dyn UserRepository + Send + Sync>,
            token_manager.clone() as Arc<dyn TokenManager>,
            password_manager.clone() as Arc<dyn PasswordManagerTrait>,
        ));

        Arc::new(Self {
            signup_usecase,
            signin_usecase,
        })
    }
}

macro_rules! impl_from_ref {
    ($name:ident, $field:ident) => {
        impl axum::extract::FromRef<AppState> for Arc<dyn $name + Send + Sync> {
            fn from_ref(state: &AppState) -> Self {
                state.0.$field.clone()
            }
        }
    };
}
impl_from_ref!(SignUpUseCase, signup_usecase);
impl_from_ref!(SignInUseCase, signin_usecase);
