pub(crate) mod domain;
pub(crate) mod usecases;
pub(crate) mod handlers;
pub(crate) mod middleware;
pub mod infrastructure;

use std::sync::Arc;
use sqlx::PgPool;
use crate::{
    infrastructure::{
        external::tmdb::client::TmdbClient,
        persistence::postgres::user::PostgresUserRepository,
        security::{
            password::PasswordManager,
            token::JwtTokenManager
        }
    },
    usecases::{
        gateway::tmdb::TmdbGateway,
        interactor::{
            sign_up::SignUpInteractor,
            sign_in::SignInInteractor,
            search::SearchInteractor,
        },
        port::{
            sign_up::SignUpUseCase,
            sign_in::SignInUseCase,
            search::SearchUseCase,
        },
        repository::{
            user::UserRepository
        },
        security::{
            password::PasswordManager as PasswordManagerTrait,
            token::TokenManager
        }
    }
};

pub struct AppRegistry {
    pub(crate) signup_usecase: Arc<dyn SignUpUseCase + Send + Sync>,
    pub(crate) signin_usecase: Arc<dyn SignInUseCase + Send + Sync>,
    pub(crate) search_usecase: Arc<dyn SearchUseCase + Send + Sync>,
    pub(crate) token_manager: Arc<JwtTokenManager>,
}

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<AppRegistry>);

impl AppRegistry {
    pub async fn build(pool: PgPool) -> Arc<Self> {
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");
        let tmdb_api_key = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");
        let tmdb_base_url = std::env::var("TMDB_BASE_URL").expect("TMDB_BASE_URL must be set");

        let user_repository = Arc::new(PostgresUserRepository::new(pool));
        let password_manager = Arc::new(PasswordManager::new());
        let token_manager = Arc::new(JwtTokenManager::new(jwt_secret));
        let tmdb_gateway = Arc::new(TmdbClient::new(tmdb_api_key, tmdb_base_url));

        let signup_usecase = Arc::new(SignUpInteractor::new(
            user_repository.clone() as Arc<dyn UserRepository + Send + Sync>,
            password_manager.clone() as Arc<dyn PasswordManagerTrait>,
        ));
        let signin_usecase = Arc::new(SignInInteractor::new(
            user_repository.clone() as Arc<dyn UserRepository + Send + Sync>,
            token_manager.clone() as Arc<dyn TokenManager>,
            password_manager.clone() as Arc<dyn PasswordManagerTrait>,
        ));
        let search_usecase = Arc::new(SearchInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));

        Arc::new(Self {
            signup_usecase,
            signin_usecase,
            search_usecase,
            token_manager,
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
impl_from_ref!(SearchUseCase, search_usecase);

impl axum::extract::FromRef<AppState> for Arc<JwtTokenManager> {
    fn from_ref(state: &AppState) -> Self {
        state.0.token_manager.clone()
    }
}
