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
        let (
            id,
            episode_number,
            season_number,
            title,
            overview,
            runtime,
            still_path,
            air_date,
            vote_average,
            production_code,
            episode_type,
        ) = entity.into_parts();

        Self {
            id,
            episode_number,
            season_number,
            title,
            overview,
            runtime,
            still_path,
            air_date,
            vote_average,
            production_code,
            episode_type,
        }
    }
}
