use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::tv_series::TvSeries,
    usecases::dto::genre::Genre
};

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

impl From<TvSeries> for TvSeriesOutput {
    fn from(entity: TvSeries) -> Self {
        Self {
            id: entity.id(),
            title: entity.title().to_string(),
            original_title: entity.original_title().to_string(),
            overview: entity.overview().clone(),
            number_of_seasons: entity.number_of_seasons(),
            number_of_episodes: entity.number_of_episodes(),
            poster_path: entity.poster_path().clone(),
            backdrop_path: entity.backdrop_path().clone(),
            first_air_date: entity.first_air_date().clone(),
            vote_average: entity.vote_average(),
            tagline: entity.tagline().clone(),
            genres: entity.genres().into_iter().map(|g| {
                Genre { id: g.id(), name: g.name().to_string() }
            }).collect(),
            season_summaries: entity.season_summaries().into_iter().map(|s| {
                SeasonSummary {
                    id: s.id(),
                    season_number: s.season_number(),
                    episode_count: s.episode_count(),
                    title: s.title().to_string(),
                    overview: s.overview().clone(),
                    poster_path: s.poster_path().clone(),
                    air_date: s.air_date().clone(),
                    vote_average: s.vote_average()
                }
            }).collect()
        }
    }
}
