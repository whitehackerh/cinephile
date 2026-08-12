use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct TvSeasonInput {
    pub series_id: i32,
    pub season_number: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSeasonOutput {
    pub id: i32,
    pub season_number: i32,
    pub episode_count: i32,
    pub title: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub episode_summaries: Vec<EpisodeSummary>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EpisodeSummary {
    pub id: i32,
    pub episode_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub runtime: Option<i32>,
    pub poster_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
}