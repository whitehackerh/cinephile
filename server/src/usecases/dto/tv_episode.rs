use serde::{Deserialize, Serialize};

use crate::domain::entities::tv_episode::TvEpisode;

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

impl From<TvEpisode> for TvEpisodeOutput {
    fn from(entity: TvEpisode) -> Self {
        Self {
            id: entity.id(),
            episode_number: entity.episode_number(),
            season_number: entity.season_number(),
            title: entity.title().to_string(),
            overview: entity.overview().clone(),
            runtime: entity.runtime(),
            still_path: entity.still_path().clone(),
            air_date: entity.air_date().clone(),
            vote_average: entity.vote_average(),
            production_code: entity.production_code().clone(),
            episode_type: entity.episode_type().clone()
        }
    }
}
