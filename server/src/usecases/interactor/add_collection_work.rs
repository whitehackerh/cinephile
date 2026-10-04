use async_trait::async_trait;
use futures::future::try_join_all;
use std::sync::Arc;
use tokio::try_join;

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
        dto::{
            add_collection_work::{
                AddCollectionWorkInput,
                AddCollectionWorkOutput,
            },
            collection_work::CollectionWork as CollectionWorkDto
        },
        gateway::tmdb::TmdbGateway,
        port::{
            add_collection_work::AddCollectionWorkUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        repository::collection::CollectionRepository,
        shared::target_path_parser::TargetPathParser
    },
};

pub(crate) struct AddCollectionWorkInteractor {
    collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl AddCollectionWorkInteractor {
    pub fn new(
        collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        uow: Arc<dyn UnitOfWork>) -> Self {
        Self { 
            collection_repository,
            tmdb_gateway,
            uow
        }
    }

    async fn fetch_work(&self, target_path: &str, work_type: &str) -> Result<Work, AppError> {
        match work_type {
            "movie" => {
                let id = TargetPathParser::extract_movie_id(target_path)?;
                Ok(Work::Movie(self.tmdb_gateway.fetch_movie_by_id(id).await?))
            }
            "series" => {
                let id = TargetPathParser::extract_series_id(target_path)?;
                Ok(Work::TvSeries(self.tmdb_gateway.fetch_tv_series_by_id(id).await?))
            }
            "season" => {
                let (series_id, season_no) = TargetPathParser::extract_season_params(target_path)?;
                Ok(Work::TvSeason(self.tmdb_gateway.fetch_tv_season(series_id, season_no).await?))
            }
            "episode" => {
                let (series_id, season_no, episode_no) = TargetPathParser::extract_episode_params(target_path)?;
                Ok(Work::TvEpisode(self.tmdb_gateway.fetch_tv_episode(series_id, season_no, episode_no).await?))
            }
            _ => Err(AppError::Validation("Invalid work_type".into())),
        }
    }
}

#[async_trait]
impl AddCollectionWorkUseCase for AddCollectionWorkInteractor {
    async fn execute(&self, input: AddCollectionWorkInput) -> Result<AddCollectionWorkOutput, AppError> {
        let collection_for_reconstruct = self.collection_repository
            .find_by_id_with_works(&input.id, &input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::EntityNotFound("Review not found".into()))?;

        if collection_for_reconstruct
            .works
            .iter()
            .any(|w| w.target_path == input.target_path)
        {
            return Err(AppError::Validation(
                "This work is already in the collection".into(),
            ));
        }

        let existing_works_future = try_join_all(
            collection_for_reconstruct.works.iter().map(|w| async {
                let work = self.fetch_work(&w.target_path, &w.work_type).await?;
                Ok(CollectionWork::reconstruct(
                    w.id,
                    w.target_path.clone(),
                    work,
                    w.added_at,
                ))
            })
        );

        let new_work_future = async {
            let work = self.fetch_work(&input.target_path, &input.work_type).await?;
            Ok(CollectionWork::new(
                input.target_path.clone(),
                work,
            ))
        };

        let (existing_works, new_collection_work) = try_join!(
            existing_works_future,
            new_work_future
        )?;

        let collection: Collection = self.uow.execute(move |repos| {
            Box::pin(async move {
                let locked_collection_for_reconstruct = repos.collection_repo
                    .find_by_id_with_works_for_update(&input.id, &input.user_id)
                    .await?
                    .ok_or_else(|| AppError::EntityNotFound("Collection not found".into()))?;

                let mut collection = Collection::reconstruct(
                    locked_collection_for_reconstruct.id,
                    locked_collection_for_reconstruct.user_id,
                    locked_collection_for_reconstruct.title,
                    locked_collection_for_reconstruct.description,
                    locked_collection_for_reconstruct.cover_image_path,
                    existing_works,
                    locked_collection_for_reconstruct.created_at,
                    locked_collection_for_reconstruct.updated_at,
                );

                collection.add_work(new_collection_work.clone());

                repos.collection_repo.update(&collection).await?;
                repos.collection_work_repo.create(&new_collection_work, &collection.id()).await?;

                Ok(collection)
            })
        })
        .await
        .map_err(|e| AppError::Infrastructure(e.to_string()))?;

         Ok(AddCollectionWorkOutput {
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
            updated_at: collection.updated_at(),
        })
    }
}
