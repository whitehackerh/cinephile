use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::movie::Movie,
    usecases::dto::genre::Genre,
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
        let (
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
            genres,
        ) = entity.into_parts();

        Self {
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
            genres: genres.into_iter().map(Into::into).collect(),
        }
    }
}
