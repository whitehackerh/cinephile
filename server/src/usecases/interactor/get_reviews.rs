use async_trait::async_trait;
use futures::future::join_all;
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
        dto::{
            get_reviews::{
                GetReviewsInput,
                GetReviewsOutput,
            },
            review::Review as ReviewDto
        },
        gateway::tmdb::TmdbGateway,
        port::get_reviews::GetReviewsUseCase,
        repository::review::ReviewRepository,
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct GetReviewsInteractor {
    review_repository: Arc<dyn ReviewRepository + Send + Sync>,
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl GetReviewsInteractor {
    pub fn new(
        review_repository: Arc<dyn ReviewRepository + Send + Sync>,
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    ) -> Self {
        Self { review_repository, tmdb_gateway }
    }
}

#[async_trait]
impl GetReviewsUseCase for GetReviewsInteractor {
    async fn execute(&self, input: GetReviewsInput) -> Result<GetReviewsOutput, AppError> {
        let review_without_work_list = self.review_repository.fetch_all(&input.user_id)
            .await
            .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        let total_count = review_without_work_list.len();
        if total_count == 0 {
            return Ok(vec![]);
        }

        let tasks = review_without_work_list.into_iter().map(|review_without_work| async move {
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
                review_without_work.id,
                input.user_id,
                review_without_work.rating,
                review_without_work.content,
                review_without_work.work_title,
                review_without_work.target_path,
                work,
                review_without_work.created_at,
                review_without_work.updated_at,
                review_without_work.deleted_at,
            );

            Ok::<Review, AppError>(review)
        });

        let results = join_all(tasks).await;
        let reviews: Vec<Review> = results.into_iter().collect::<Result<Vec<_>, _>>()?;

        Ok(reviews
            .into_iter()
            .map(|r| ReviewDto {
                id: r.id(),
                rating: r.rating(),
                content: r.content().clone(),
                work_type: r.work_type().to_string(),
                work_title: r.work_title().to_string(),
                target_path: r.target_path().to_string(),
                work: r.work().clone().into(),
                created_at: r.created_at(),
                updated_at: r.updated_at(),
                deleted_at: r.deleted_at(),
            })
            .collect())
    }
}
