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
            collection::PostgresCollectionRepository,
            collection_work::PostgresCollectionWorkRepository,
            review::PostgresReviewRepository,
            unit_of_work::PostgresUnitOfWork,
            user::PostgresUserRepository
        },
        security::{
            password::PasswordManager,
            token::JwtTokenManager
        }
    }, usecases::{
        gateway::tmdb::TmdbGateway,
        interactor::{
            add_collection_work::AddCollectionWorkInteractor,
            delete_reviews::DeleteReviewsInteractor,
            get_review::GetReviewInteractor,
            get_reviews::GetReviewsInteractor,
            movie::MovieInteractor,
            patch_reviews::PatchReviewsInteractor,
            create_collection::CreateCollectionInteractor,
            create_review::CreateReviewInteractor,
            search::SearchInteractor,
            sign_in::SignInInteractor,
            sign_up::SignUpInteractor,
            tv_episode::TvEpisodeInteractor,
            tv_season::TvSeasonInteractor,
            tv_series::TvSeriesInteractor
        },
        port::{
            add_collection_work::AddCollectionWorkUseCase,
            delete_reviews::DeleteReviewsUseCase,
            get_review::GetReviewUseCase,
            get_reviews::GetReviewsUseCase,
            movie::MovieUseCase,
            patch_reviews::PatchReviewsUseCase,
            create_collection::CreateCollectionUseCase,
            create_review::CreateReviewUseCase,
            search::SearchUseCase,
            sign_in::SignInUseCase,
            sign_up::SignUpUseCase,
            tv_episode::TvEpisodeUseCase,
            tv_season::TvSeasonUseCase,
            tv_series::TvSeriesUseCase,
            unit_of_work::UnitOfWork
        },
        repository::{
            collection::CollectionRepository,
            collection_work::CollectionWorkRepository,
            review::ReviewRepository,
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
    pub(crate) movie_usecase: Arc<dyn MovieUseCase + Send + Sync>,
    pub(crate) tv_episode_usecase: Arc<dyn TvEpisodeUseCase + Send + Sync>,
    pub(crate) tv_season_usecase: Arc<dyn TvSeasonUseCase + Send + Sync>,
    pub(crate) tv_series_usecase: Arc<dyn TvSeriesUseCase + Send + Sync>,
    pub(crate) create_review_usecase: Arc<dyn CreateReviewUseCase + Send + Sync>,
    pub(crate) patch_reviews_usecase: Arc<dyn PatchReviewsUseCase + Send + Sync>,
    pub(crate) delete_reviews_usecase: Arc<dyn DeleteReviewsUseCase + Send + Sync>,
    pub(crate) get_reviews_usecase: Arc<dyn GetReviewsUseCase + Send + Sync>,
    pub(crate) get_review_usecase: Arc<dyn GetReviewUseCase + Send + Sync>,
    pub(crate) create_collection_usecase: Arc<dyn CreateCollectionUseCase + Send + Sync>,
    pub(crate) add_collection_work_usecase: Arc<dyn AddCollectionWorkUseCase + Send + Sync>,
    pub(crate) token_manager: Arc<JwtTokenManager>
}

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<AppRegistry>);

impl AppRegistry {
    pub async fn build(pool: PgPool) -> Arc<Self> {
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");
        let tmdb_api_key = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");
        let tmdb_base_url = std::env::var("TMDB_BASE_URL").expect("TMDB_BASE_URL must be set");

        let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let review_repository = Arc::new(PostgresReviewRepository::new(pool.clone()));
        let collection_repository = Arc::new(PostgresCollectionRepository::new(pool.clone()));
        let collection_work_repository = Arc::new(PostgresCollectionWorkRepository::new(pool.clone()));
        
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
        let create_review_usecase = Arc::new(CreateReviewInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>,
            uow.clone()
        ));
        let patch_reviews_usecase = Arc::new(PatchReviewsInteractor::new(
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>,
            uow.clone()
        ));
        let delete_reviews_usecase = Arc::new(DeleteReviewsInteractor::new(
            review_repository.clone() as Arc<dyn ReviewRepository + Send + Sync>,
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>,
            uow.clone()
        ));
        let get_reviews_usecase = Arc::new(GetReviewsInteractor::new(
            review_repository.clone() as Arc<dyn ReviewRepository + Send + Sync>,
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let get_review_usecase = Arc::new(GetReviewInteractor::new(
            review_repository.clone() as Arc<dyn ReviewRepository + Send + Sync>,
            tmdb_gateway.clone() as Arc<dyn TmdbGateway + Send + Sync>
        ));
        let create_collection_usecase = Arc::new(CreateCollectionInteractor::new(
            uow.clone()
        ));
        let add_collection_work_usecase = Arc::new(AddCollectionWorkInteractor::new(
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
            create_review_usecase,
            patch_reviews_usecase,
            delete_reviews_usecase,
            get_reviews_usecase,
            get_review_usecase,
            create_collection_usecase,
            add_collection_work_usecase,
            token_manager
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
impl_from_ref!(CreateReviewUseCase, create_review_usecase);
impl_from_ref!(PatchReviewsUseCase, patch_reviews_usecase);
impl_from_ref!(DeleteReviewsUseCase, delete_reviews_usecase);
impl_from_ref!(GetReviewsUseCase, get_reviews_usecase);
impl_from_ref!(GetReviewUseCase, get_review_usecase);
impl_from_ref!(CreateCollectionUseCase, create_collection_usecase);
impl_from_ref!(AddCollectionWorkUseCase, add_collection_work_usecase);
