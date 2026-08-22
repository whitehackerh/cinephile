use serde::{Deserialize, Serialize};

use crate::usecases::dto::genre::Genre;

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
