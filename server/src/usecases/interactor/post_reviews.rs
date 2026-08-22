use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::errors::AppError,
    usecases::{
        dto::post_reviews::{
            PostReviewsInput,
            PostReviewsOutput,
        },
        gateway::tmdb::TmdbGateway,
        port::post_reviews::PostReviewsUseCase,
        repository::review::ReviewRepository
    }
};

pub(crate) struct PostReviewsInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
    review_repository: Arc<dyn ReviewRepository + Send + Sync>,
}

impl PostReviewsInteractor {
    pub fn new(
        tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
        review_repository: Arc<dyn ReviewRepository + Send + Sync>
    ) -> Self {
        Self { tmdb_gateway, review_repository }
    }
}

// #[async_trait]
// impl PostReviewsUseCase for PostReviewsInteractor {
//     async fn execute(&self, input: PostReviewsInput) -> Result<PostReviewsOutput, AppError> {
//         // match input.work_type {
//         //     "movie" => self.tmdb_gateway.fetch_movie_by_id(input.)
//         // }
//        // tmdb api
//        // entity
//        // insert
//        Ok(PostReviewsOutput {
            
//        })
//     }
// }
