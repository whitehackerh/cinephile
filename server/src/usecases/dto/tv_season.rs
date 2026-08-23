use serde::{Deserialize, Serialize};

use crate::domain::entities::tv_season::TvSeason;

#[derive(Debug)]
pub(crate) struct TvSeasonInput {
    pub series_id: i32,
    pub season_number: i32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct EpisodeSummary {
    pub id: i32,
    pub episode_number: i32,
    pub title: String,
    pub overview: Option<String>,
    pub runtime: Option<i32>,
    pub still_path: Option<String>,
    pub air_date: Option<String>,
    pub vote_average: Option<f64>,
}

impl From<TvSeason> for TvSeasonOutput {
    fn from(entity: TvSeason) -> Self {
        Self {
            id: entity.id(),
            season_number: entity.season_number(),
            episode_count: entity.episode_count(),
            title: entity.title().to_string(),
            overview: entity.overview().clone(),
            poster_path: entity.poster_path().clone(),
            air_date: entity.air_date().clone(),
            vote_average: entity.vote_average(),
            episode_summaries: entity.episode_summaries().into_iter().map(|e| {
                EpisodeSummary {
                    id: e.id(),
                    episode_number: e.episode_number(),
                    title: e.title().to_string(),
                    overview: e.overview().clone(),
                    runtime: e.runtime(),
                    still_path: e.still_path().clone(),
                    air_date: e.air_date().clone(),
                    vote_average: e.vote_average()
                }
            }).collect()
        }
    }
}
