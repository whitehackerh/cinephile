use async_trait::async_trait;
use std::sync::Arc;
use chrono::Utc;

use crate::{
    domain::{
        entities::{
            review::Review,
            work::Work
        },
        errors::AppError
    },
    usecases::{
        dto::patch_reviews::{
            PatchReviewsInput,
            PatchReviewsOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::{
            patch_reviews::PatchReviewsUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        repository::review::ReviewRepository,
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct PatchReviewsInteractor {
    review_repository: Arc<dyn ReviewRepository + Send + Sync>,
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl PatchReviewsInteractor {
    pub fn new(
        review_repository: Arc<dyn ReviewRepository + Send + Sync>,
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        uow: Arc<dyn UnitOfWork>
    ) -> Self {
        Self { review_repository, tmdb_gateway, uow }
    }
}

#[async_trait]
impl PatchReviewsUseCase for PatchReviewsInteractor {
    async fn execute(&self, input: PatchReviewsInput) -> Result<PatchReviewsOutput, AppError> {
        let review_without_work = self.review_repository
            .find_by_id(&input.id, &input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::EntityNotFound("Review not found".into()))?;
        
        let work = match review_without_work.work_type.as_str() {
            "movie" => {
                let id = TargetPathParser::extract_movie_id(&review_without_work.target_path)?;
                Work::Movie(self.tmdb_gateway.fetch_movie_by_id(id).await?)
            }
            "series" => {
                let id = TargetPathParser::extract_series_id(&review_without_work.target_path)?;
                Work::TvSeries(self.tmdb_gateway.fetch_tv_series_by_id(id).await?)
            }
            "season" => {
                let (series_id, season_no) = TargetPathParser::extract_season_params(&review_without_work.target_path)?;
                Work::TvSeason(self.tmdb_gateway.fetch_tv_season(series_id, season_no).await?)
            }
            "episode" => {
                let (series_id, season_no, episode_no) = TargetPathParser::extract_episode_params(&review_without_work.target_path)?;
                Work::TvEpisode(self.tmdb_gateway.fetch_tv_episode(series_id, season_no, episode_no).await?)
            }
            _ => return Err(AppError::Validation("Invalid work_type".into())),
        };

        let review = self.uow.execute(move |repos| {
            Box::pin(async move {
                let locked_review_without_work = repos.review_repo
                    .find_by_id_for_update(&input.id, &input.user_id)
                    .await?
                    .ok_or_else(|| AppError::EntityNotFound("Review not found".into()))?;

                let review = Review::reconstruct(
                    input.id,
                    input.user_id,
                    input.rating,
                    input.content,
                    locked_review_without_work.target_path,
                    work,
                    locked_review_without_work.created_at,
                    Utc::now(),
                    locked_review_without_work.deleted_at
                );

                repos.review_repo.update(&review).await?;

                Ok(review)
            })
        })
        .await
        .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        Ok(PatchReviewsOutput {
            id: review.id(),
            rating: review.rating(),
            content: review.content().clone(),
            work_type: review.work_type().to_string(),
            target_path: review.target_path().to_string(),
            work: review.work().clone().into(),
            created_at: review.created_at(),
            updated_at: review.updated_at(),
            deleted_at: review.deleted_at()
        })
    }
}
