use chrono::NaiveDate;

use crate::{
    generated::api_schema::TvEpisode,
    usecases::dto::tv_episode::TvEpisodeOutput
};

pub struct TvEpisodePresenter;

impl TvEpisodePresenter {
    pub fn to_response(output: TvEpisodeOutput) -> TvEpisode {
        TvEpisode {
            air_date: output.air_date.and_then(|a| NaiveDate::parse_from_str(&a, "%Y-%m-%d").ok()),
            episode_number: output.episode_number as i64,
            episode_type: output.episode_type,
            id: output.id as i64,
            overview: output.overview,
            production_code: output.production_code,
            runtime: output.runtime.map(|r| r as i64),
            season_number: output.season_number as i64,
            still_path: output.still_path,
            title: output.title,
            vote_average: output.vote_average
        }
    }
}
