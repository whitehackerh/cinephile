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
        dto::{
            get_collection::{
                GetCollectionInput,
                GetCollectionOutput
            },
            collection_work::CollectionWork as CollectionWorkDto
        },
        gateway::tmdb::TmdbGateway,
        port::get_collection::GetCollectionUseCase,
        repository::collection::CollectionRepository,
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct GetCollectionInteractor {
    collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>
}

impl GetCollectionInteractor {
    pub fn new(
        collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>
    ) -> Self {
        Self { 
            collection_repository,
            tmdb_gateway
        }
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
impl GetCollectionUseCase for GetCollectionInteractor {
    async fn execute(&self, input: GetCollectionInput) -> Result<GetCollectionOutput, AppError> {
        let collection_for_reconstruct = self.collection_repository
            .find_by_id_with_works(&input.id, &input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::EntityNotFound("Collection not found".into()))?;

        let existing_works = try_join_all(
            collection_for_reconstruct.works.iter().map(|w| {
                let target_path = w.target_path.clone();
                let work_type = w.work_type.clone();
                let id = w.id;
                let added_at = w.added_at;
                let tmdb_gateway = Arc::clone(&self.tmdb_gateway);

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

        let collection = Collection::reconstruct(
            collection_for_reconstruct.id,
            input.user_id,
            collection_for_reconstruct.title,
            collection_for_reconstruct.description,
            collection_for_reconstruct.cover_image_path,
            existing_works,
            collection_for_reconstruct.created_at,
            collection_for_reconstruct.updated_at
        );

        Ok(GetCollectionOutput {
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