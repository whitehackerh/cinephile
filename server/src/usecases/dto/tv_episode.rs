use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct TvEpisodeInput {
    pub series_id: i32,
    pub season_number: i32,
    pub episode_number: i32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TvEpisodeOutput {
    pub id: i32,
    pub episode_number: i32,
    pub season_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub production_code: Option<String>,
    pub episode_type: Option<String>,
}
