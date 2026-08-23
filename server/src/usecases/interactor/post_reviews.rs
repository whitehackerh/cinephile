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
        dto::post_reviews::{
            PostReviewsInput,
            PostReviewsOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::{
            post_reviews::PostReviewsUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        shared::target_path_parser::TargetPathParser
    }
};

pub(crate) struct PostReviewsInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    uow: Arc<dyn UnitOfWork>
}

impl PostReviewsInteractor {
    pub fn new(
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        uow: Arc<dyn UnitOfWork>
    ) -> Self {
        Self { tmdb_gateway, uow }
    }
}

#[async_trait]
impl PostReviewsUseCase for PostReviewsInteractor {
    async fn execute(&self, input: PostReviewsInput) -> Result<PostReviewsOutput, AppError> {
        let work = match input.work_type.as_str() {
            "movie" => {
                let id = TargetPathParser::extract_movie_id(&input.target_path)?;
                Work::Movie(self.tmdb_gateway.fetch_movie_by_id(id as i32).await?)
            }
            "series" => {
                let id = TargetPathParser::extract_series_id(&input.target_path)?;
                Work::TvSeries(self.tmdb_gateway.fetch_tv_series_by_id(id as i32).await?)
            }
            "season" => {
                let (series_id, season_no) = TargetPathParser::extract_season_params(&input.target_path)?;
                Work::TvSeason(self.tmdb_gateway.fetch_tv_season(series_id as i32, season_no as i32).await?)
            }
            "episode" => {
                let (series_id, season_no, episode_no) = TargetPathParser::extract_episode_params(&input.target_path)?;
                Work::TvEpisode(self.tmdb_gateway.fetch_tv_episode(series_id as i32, season_no as i32, episode_no as i32).await?)
            }
            _ => return Err(AppError::Validation("Invalid work_type".into())),
        };

        let review = Review::new(
            input.user_id,
            input.rating,
            input.content,
            input.target_path,
            work
        )?;

        let cloned_review = review.clone();
        let user_id = review.user_id();
        let target_path = review.target_path().to_string();

        self.uow.execute(move |repos| {
            Box::pin(async move {
                let exists = repos
                    .review_repo
                    .exists_by_user_and_target(&user_id, &target_path)
                    .await?;

                if exists {
                    return Err(anyhow::anyhow!("REVIEW_ALREADY_EXISTS"));
                }

                repos.review_repo.create(&cloned_review).await?;
                Ok(())
            })
        })
        .await
        .map_err(|e| {
            if e.to_string().contains("REVIEW_ALREADY_EXISTS") {
                AppError::Conflict("Review already exists for this work".into())
            } else {
                AppError::Infrastructure(e.to_string())
            }
        })?;

       Ok(PostReviewsOutput {
            id: review.id(),
            rating: review.rating(),
            content: review.content().clone(),
            work_type: review.work_type().to_string(),
            target_path: review.target_path().to_string(),
            work: review.work().clone().into(),
            created_at: review.created_at(),
            updated_at: review.updated_at(),
            deleted_at: review.deleted_at().clone()
       })
    }
}
