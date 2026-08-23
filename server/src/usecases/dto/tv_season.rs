use serde::{Deserialize, Serialize};

use crate::domain::entities::{
    tv_episode_summary::TvEpisodeSummary,
    tv_season::TvSeason
};

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

impl From<TvEpisodeSummary> for EpisodeSummary {
    fn from(entity: TvEpisodeSummary) -> Self {
        let (
            id,
            episode_number,
            title,
            overview,
            runtime,
            still_path,
            air_date,
            vote_average,
        ) = entity.into_parts();

        Self {
            id,
            episode_number,
            title,
            overview,
            runtime,
            still_path,
            air_date,
            vote_average,
        }
    }
}

impl From<TvSeason> for TvSeasonOutput {
    fn from(entity: TvSeason) -> Self {
        let (
            id,
            season_number,
            episode_count,
            title,
            overview,
            poster_path,
            air_date,
            vote_average,
            episode_summaries,
        ) = entity.into_parts();

        Self {
            id,
            season_number,
            episode_count,
            title,
            overview,
            poster_path,
            air_date,
            vote_average,
            episode_summaries: episode_summaries.into_iter().map(Into::into).collect(),
        }
    }
}
