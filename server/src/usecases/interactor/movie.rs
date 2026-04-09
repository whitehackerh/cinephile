use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        errors::AppError,
    },
    usecases::{
        dto::movie::{
            MovieInput,
            MovieOutput,
            Genre
        },
        gateway::tmdb::TmdbGateway,
        port::movie::MovieUseCase,
    }
};

pub(crate) struct MovieInteractor {
    tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>,
}

impl MovieInteractor {
    pub fn new(tmdb_gateway: Arc<dyn TmdbGateway + Send + Sync>) -> Self {
        Self { tmdb_gateway }
    }
}

#[async_trait]
impl MovieUseCase for MovieInteractor {
    async fn execute(&self, input: MovieInput) -> Result<MovieOutput, AppError> {
        if input.id < 1 {
            return Err(AppError::Validation("Id must be greater than or equal to 1".to_string()));
        }

        let movie = self.tmdb_gateway
            .fetch_movie_by_id(input.id)
            .await?;

        Ok(MovieOutput {
            id: movie.id(),
            title: movie.title().to_string(),
            original_title: movie.original_title().to_string(),
            over_view: movie.over_view().clone(),
            poster_path: movie.poster_path().clone(),
            backdrop_path: movie.backdrop_path().clone(),
            release_date: movie.release_date().clone(),
            runtime: movie.runtime(),
            vote_average: movie.vote_average(),
            tagline: movie.tagline().clone(),
            genres: movie.genres().iter()
                .map(|genre| Genre { id: genre.id(), name: genre.name().to_string() })
                .collect(),
        })
    }
}
