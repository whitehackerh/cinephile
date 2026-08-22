use serde::{Deserialize, Serialize};

use crate::usecases::dto::genre::Genre;

#[derive(Debug)]
pub(crate) struct TvSeriesInput {
    pub id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TvSeriesOutput {
    pub id: i32,
    pub title: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub number_of_seasons: Option<i32>,
    pub number_of_episodes: Option<i32>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub tagline: Option<String>,
    pub genres: Vec<Genre>,
    pub season_summaries: Vec<SeasonSummary>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SeasonSummary {
    pub id: i32,
    pub season_number: i32,
    pub episode_count: i32,
    pub title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
}