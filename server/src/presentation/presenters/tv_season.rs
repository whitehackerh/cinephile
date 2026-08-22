use chrono::NaiveDate;

use crate::{
    generated::api_schema::{
        TvEpisodeSummary,
        TvSeason
    },
    usecases::dto::tv_season::TvSeasonOutput
};

pub struct TvSeasonPresenter;

impl TvSeasonPresenter {
    pub fn to_response(output: TvSeasonOutput) -> TvSeason {
        TvSeason {
            air_date: output.air_date.and_then(|a| NaiveDate::parse_from_str(&a, "%Y-%m-%d").ok()),
            episode_count: output.episode_count as i64,
            episode_summaries: output.episode_summaries.into_iter().map(
                |e| TvEpisodeSummary {
                    air_date: e.air_date.and_then(|a| NaiveDate::parse_from_str(&a, "%Y-%m-%d").ok()),
                    episode_number: e.episode_number as i64,
                    id: e.id as i64,
                    overview: e.overview,
                    runtime: e.runtime.map(|r| r as i64),
                    still_path: e.still_path,
                    title: e.title,
                    vote_average: e.vote_average
                }
            ).collect(),
            id: output.id as i64,
            overview: output.overview,
            poster_path: output.poster_path,
            season_number: output.season_number as i64,
            title: output.title,
            vote_average: output.vote_average
        }
    }
}
