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
            Genre as GenreDto,
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

        let (
            id, title, original_title, overview, poster_path, 
            backdrop_path, release_date, runtime, vote_average, 
            tagline, genres
        ) = movie.into_parts();

        Ok(MovieOutput {
            id,
            title,
            original_title,
            overview,
            poster_path,
            backdrop_path,
            release_date,
            runtime,
            vote_average,
            tagline,
            genres: genres
                .into_iter()
                .map(|g| {
                    let (g_id, g_name) = g.into_parts();
                    GenreDto { id: g_id, name: g_name }
                })
                .collect(),
        })
    }
}
