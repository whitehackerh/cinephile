use async_trait::async_trait;
use futures::future::try_join_all;
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            collection::Collection,
            collection_work::CollectionWork,
            work::Work
        },
        errors::AppError
    },
    usecases::{
        dto::{
            collection_work::CollectionWork as CollectionWorkDto,
            remove_collection_work::{
                RemoveCollectionWorkInput,
                RemoveCollectionWorkOutput
            }
        },
        gateway::tmdb::TmdbGateway,
        port::{
            remove_collection_work::RemoveCollectionWorkUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct RemoveCollectionWorkInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl RemoveCollectionWorkInteractor {
    pub fn new(
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        uow: Arc<dyn UnitOfWork>
    ) -> Self {
        Self { tmdb_gateway, uow }
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
impl RemoveCollectionWorkUseCase for RemoveCollectionWorkInteractor {
    async fn execute(&self, input: RemoveCollectionWorkInput) -> Result<RemoveCollectionWorkOutput, AppError> {
        let tmdb_gateway = Arc::clone(&self.tmdb_gateway);

        let collection: Collection = self.uow.execute(move |repos| {
            Box::pin(async move {
                let collection_for_reconstruct = repos.collection_repo
                    .find_by_id_with_works_for_update(&input.id, &input.user_id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!(AppError::EntityNotFound("Collection not found".into())))?;

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

                            Ok::<CollectionWork, anyhow::Error>(CollectionWork::reconstruct(
                                id,
                                target_path,
                                work,
                                added_at
                            ))
                        }
                    })
                ).await?;
                
                let mut collection = Collection::reconstruct(
                    collection_for_reconstruct.id,
                    input.user_id,
                    collection_for_reconstruct.title,
                    collection_for_reconstruct.description,
                    collection_for_reconstruct.cover_image_path,
                    works,
                    collection_for_reconstruct.created_at,
                    collection_for_reconstruct.updated_at
                );

                collection.remove_work(input.collection_work_id.clone());

                repos.collection_work_repo.delete(&input.collection_work_id).await?;

                repos.collection_repo.update(&collection).await?;

                Ok(collection)
            })
        })
        .await
        .map_err(|e| {
            match e.downcast::<AppError>() {
                Ok(app_err) => app_err,
                Err(other_err) => AppError::Infrastructure(other_err.to_string()),
            }
        })?;

        Ok(RemoveCollectionWorkOutput {
            id: collection.id(),
            title: collection.title().to_string(),
            description: collection.description().clone(),
            cover_image_path: collection.cover_image_path().clone(),
            works: collection.works()
                .into_iter()
                .map(|w| CollectionWorkDto {
                    id: w.id(),
                    target_path: w.target_path().to_string(),
                    work_type: w.work_type().to_string(),
                    work: w.work().clone().into(),
                    added_at: w.added_at()
                })
                .collect(),
            created_at: collection.created_at(),
            updated_at: collection.updated_at()
        })
    }
}
