use async_trait::async_trait;
use futures::future::try_join_all;
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            collection_work::CollectionWork,
            collection::Collection,
            work::Work
        },
        errors::AppError
    },
    usecases::{
        dto::delete_collection::DeleteCollectionInput,
        gateway::tmdb::TmdbGateway,
        port::{
            delete_collection::DeleteCollectionUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        repository::collection::CollectionRepository,
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct DeleteCollectionInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl DeleteCollectionInteractor {
    pub fn new(
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
        uow: Arc<dyn UnitOfWork>
    ) -> Self {
        Self { tmdb_gateway, collection_repository, uow }
    }

    async fn fetch_work(
        target_path: &str,
        work_type: &str,
        tmdb_gateway: &Arc<dyn TmdbGateway + Send + Sync>
    ) -> Result<Work, AppError> {
        match work_type {
            "movie" => {
                let id = TargetPathParser::extract_movie_id(target_path)?;
                Ok(Work::Movie(tmdb_gateway.fetch_movie_by_id(id).await?))
            }
            "series" => {
                let id = TargetPathParser::extract_series_id(target_path)?;
                Ok(Work::TvSeries(tmdb_gateway.fetch_tv_series_by_id(id).await?))
            }
            "season" => {
                let (series_id, season_no) = TargetPathParser::extract_season_params(target_path)?;
                Ok(Work::TvSeason(tmdb_gateway.fetch_tv_season(series_id, season_no).await?))
            }
            "episode" => {
                let (series_id, season_no, episode_no) = TargetPathParser::extract_episode_params(target_path)?;
                Ok(Work::TvEpisode(tmdb_gateway.fetch_tv_episode(series_id, season_no, episode_no).await?))
            }
            _ => Err(AppError::Validation("Invalid work_type".into()))
        }
    }
}

#[async_trait]
impl DeleteCollectionUseCase for DeleteCollectionInteractor {
    async fn execute(&self, input: DeleteCollectionInput) -> Result<(), AppError> {
        let tmdb_gateway = Arc::clone(&self.tmdb_gateway);
        
        let collection_for_reconstruct = self.collection_repository
            .find_by_id_with_works(&input.id, &input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::EntityNotFound("Collection not found".into()))?;
        
        let works = try_join_all(
            collection_for_reconstruct.works.iter().map(|w| {
                let target_path = w.target_path.clone();
                let work_type = w.work_type.clone();
                let id = w.id;
                let added_at = w.added_at;
                let tmdb_gateway = Arc::clone(&tmdb_gateway);

                async move {
                    let work = Self::fetch_work(&target_path, &work_type, &tmdb_gateway)
                        .await?;

                    Ok::<CollectionWork, AppError>(CollectionWork::reconstruct(
                        id,
                        target_path,
                        work,
                        added_at
                    ))
                }
            })
        ).await?;

        self.uow.execute(move |repos| {
            Box::pin(async move {
                let locked_collection_with_works = repos.collection_repo
                    .find_by_id_with_works_for_update(&input.id, &input.user_id)
                    .await?
                    .ok_or_else(|| AppError::EntityNotFound("Collection not found".into()))?;

                let collection = Collection::reconstruct(
                    locked_collection_with_works.id,
                    input.user_id,
                    locked_collection_with_works.title,
                    locked_collection_with_works.description,
                    locked_collection_with_works.cover_image_path,
                    works,
                    locked_collection_with_works.created_at,
                    locked_collection_with_works.updated_at
                );

                repos.collection_repo.delete(&collection).await?;

                Ok(())
            })
        })
        .await
        .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        Ok(())
    }
}
