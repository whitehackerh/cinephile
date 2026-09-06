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
        dto::get_review::{
            GetReviewInput,
            GetReviewOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::get_review::GetReviewUseCase,
        repository::review::ReviewRepository,
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct GetReviewInteractor {
    review_repository: Arc<dyn ReviewRepository + Send + Sync>,
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl GetReviewInteractor {
    pub fn new(
        review_repository: Arc<dyn ReviewRepository + Send + Sync>,
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    ) -> Self {
        Self { review_repository, tmdb_gateway }
    }
}

#[async_trait]
impl GetReviewUseCase for GetReviewInteractor {
    async fn execute(&self, input: GetReviewInput) -> Result<GetReviewOutput, AppError> {
        let review_without_work = self.review_repository.find_by_id(&input.id, &input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?
            .ok_or_else(|| AppError::EntityNotFound("Review not found".to_string()))?;

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

        let review = Review::reconstruct(
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

        Ok(GetReviewOutput {
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
