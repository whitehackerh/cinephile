pub mod domain;
pub mod usecases;
pub mod handlers;
pub mod infrastructure;

use std::sync::Arc;
use sqlx::PgPool;
use crate::infrastructure::persistence::postgres::user::PostgresUserRepository;
use crate::usecases::interactor::sign_up::SignUpInteractor;
use crate::usecases::port::sign_up::SignUpUseCase;
use crate::usecases::repository::user::UserRepository;

pub struct AppRegistry {
    pub signup_usecase: Arc<dyn SignUpUseCase + Send + Sync>,
}

#[derive(Clone)]
pub struct AppState(pub Arc<AppRegistry>);

impl AppRegistry {
    pub async fn build(pool: PgPool) -> Arc<Self> {
        let user_repository = Arc::new(PostgresUserRepository::new(pool));
        let signup_usecase = Arc::new(SignUpInteractor::new(user_repository as Arc<dyn UserRepository + Send + Sync>));

        Arc::new(Self {
            signup_usecase,
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
