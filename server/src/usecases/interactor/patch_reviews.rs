use async_trait::async_trait;
use std::sync::Arc;

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
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct PatchReviewsInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl PatchReviewsInteractor {
    pub fn new(
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        uow: Arc<dyn UnitOfWork>
    ) -> Self {
        Self { tmdb_gateway, uow }
    }
}

#[async_trait]
impl PatchReviewsUseCase for PatchReviewsInteractor {
    async fn execute(&self, input: PatchReviewsInput) -> Result<PatchReviewsOutput, AppError> {
        let tmdb_gateway = Arc::clone(&self.tmdb_gateway);
        
        let review: Review = self.uow.execute(move |repos| {
            Box::pin(async move {
                let review_without_work = repos.review_repo
                    .find_by_id_for_update(&input.id, &input.user_id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!(AppError::EntityNotFound("Review not found".into())))?;

                let work = match review_without_work.work_type.as_str() {
                    "movie" => {
                        let id = TargetPathParser::extract_movie_id(&review_without_work.target_path)?;
                        Work::Movie(tmdb_gateway.fetch_movie_by_id(id).await?)
                    }
                    "series" => {
                        let id = TargetPathParser::extract_series_id(&review_without_work.target_path)?;
                        Work::TvSeries(tmdb_gateway.fetch_tv_series_by_id(id).await?)
                    }
                    "season" => {
                        let (series_id, season_no) = TargetPathParser::extract_season_params(&review_without_work.target_path)?;
                        Work::TvSeason(tmdb_gateway.fetch_tv_season(series_id, season_no).await?)
                    }
                    "episode" => {
                        let (series_id, season_no, episode_no) = TargetPathParser::extract_episode_params(&review_without_work.target_path)?;
                        Work::TvEpisode(tmdb_gateway.fetch_tv_episode(series_id, season_no, episode_no).await?)
                    }
                    _ => return Err(anyhow::anyhow!(AppError::Validation("Invalid work_type".into()))),
                };

                let mut review = Review::reconstruct(
                    input.id,
                    input.user_id,
                    review_without_work.rating,
                    review_without_work.content,
                    review_without_work.target_path,
                    work,
                    review_without_work.created_at,
                    review_without_work.updated_at,
                    review_without_work.deleted_at
                );
                review.update(input.rating, input.content);

                repos.review_repo.update(&review).await?;

                Ok(review)
            })
        })
        .await
        .map_err(|e| {
            match e.downcast::<AppError>() {
                Ok(app_err) => app_err,
                Err(other_err) => AppError::Infrastructure(other_err.to_string()),
            }
        })?;

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
