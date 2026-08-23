pub(crate) mod domain;
pub(crate) mod generated;
pub(crate) mod middleware;
pub(crate) mod presentation;
pub(crate) mod usecases;
pub mod infrastructure;

use std::sync::Arc;
use sqlx::PgPool;
use crate::{
    infrastructure::{
        external::tmdb::client::TmdbClient,
        persistence::postgres::{
            user::PostgresUserRepository,
            review::PostgresReviewRepository,
            unit_of_work::PostgresUnitOfWork,
        },
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
            movie::MovieInteractor,
            tv_episode::TvEpisodeInteractor,
            tv_season::TvSeasonInteractor,
            tv_series::TvSeriesInteractor,
            post_reviews::PostReviewsInteractor,
        },
        port::{
            sign_up::SignUpUseCase,
            sign_in::SignInUseCase,
            search::SearchUseCase,
            movie::MovieUseCase,
            tv_episode::TvEpisodeUseCase,
            tv_season::TvSeasonUseCase,
            tv_series::TvSeriesUseCase,
            post_reviews::PostReviewsUseCase,
            unit_of_work::UnitOfWork,
        },
        repository::{
            user::UserRepository,
            review::ReviewRepository,
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
    pub(crate) movie_usecase: Arc<dyn MovieUseCase + Send + Sync>,
    pub(crate) tv_episode_usecase: Arc<dyn TvEpisodeUseCase + Send + Sync>,
    pub(crate) tv_season_usecase: Arc<dyn TvSeasonUseCase + Send + Sync>,
    pub(crate) tv_series_usecase: Arc<dyn TvSeriesUseCase + Send + Sync>,
    pub(crate) post_reviews_usecase: Arc<dyn PostReviewsUseCase + Send + Sync>,
    pub(crate) token_manager: Arc<JwtTokenManager>,
}

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<AppRegistry>);

impl AppRegistry {
    pub async fn build(pool: PgPool) -> Arc<Self> {
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");
        let tmdb_api_key = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");
        let tmdb_base_url = std::env::var("TMDB_BASE_URL").expect("TMDB_BASE_URL must be set");

        let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        // let review_repository = Arc::new(PostgresReviewRepository::new(pool));
        let password_manager = Arc::new(PasswordManager::new());
        let token_manager = Arc::new(JwtTokenManager::new(jwt_secret));
        let tmdb_gateway = Arc::new(TmdbClient::new(tmdb_api_key, tmdb_base_url));
        let uow: Arc<dyn UnitOfWork> = Arc::new(PostgresUnitOfWork::new(pool.clone()));

        let signup_usecase = Arc::new(SignUpInteractor::new(
            uow.clone(),
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
        let movie_usecase = Arc::new(MovieInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let tv_episode_usecase = Arc::new(TvEpisodeInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let tv_season_usecase = Arc::new(TvSeasonInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let tv_series_usecase = Arc::new(TvSeriesInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let post_reviews_usecase = Arc::new(PostReviewsInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>,
            uow.clone()
        ));

        Arc::new(Self {
            signup_usecase,
            signin_usecase,
            search_usecase,
            movie_usecase,
            tv_episode_usecase,
            tv_season_usecase,
            tv_series_usecase,
            post_reviews_usecase,
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
impl_from_ref!(MovieUseCase, movie_usecase);
impl_from_ref!(TvEpisodeUseCase, tv_episode_usecase);
impl_from_ref!(TvSeasonUseCase, tv_season_usecase);
impl_from_ref!(TvSeriesUseCase, tv_series_usecase);
impl_from_ref!(PostReviewsUseCase, post_reviews_usecase);
