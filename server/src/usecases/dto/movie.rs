use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::movie::Movie,
    usecases::dto::genre::Genre
};

#[derive(Debug)]
pub(crate) struct MovieInput {
    pub id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MovieOutput {
    pub id: i32,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    pub vote_average: Option<f64>,
    pub tagline: Option<String>,
    pub genres: Vec<Genre>,
}

impl From<Movie> for MovieOutput {
    fn from(entity: Movie) -> Self {
        Self {
            id: entity.id(),
            title: entity.title().to_string(),
            original_title: entity.original_title().to_string(),
            overview: entity.overview().clone(),
            poster_path: entity.poster_path().clone(),
            backdrop_path: entity.backdrop_path().clone(),
            release_date: entity.release_date().clone(),
            runtime: entity.runtime(),
            vote_average: entity.vote_average(),
            tagline: entity.tagline().clone(),
            genres: entity.genres().into_iter().map(|g| {
                Genre { id: g.id(), name: g.name().to_string() }
            }).collect(),
        }
    }
}
