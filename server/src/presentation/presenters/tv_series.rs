use chrono::NaiveDate;

use crate::{
    generated::api_schema::{
        Genre,
        TvSeasonSummary,
        TvSeries
    },
    usecases::dto::tv_series::TvSeriesOutput
};

pub struct TvSeriesPresenter;

impl TvSeriesPresenter {
    pub fn to_response(output: TvSeriesOutput) -> TvSeries {
        TvSeries {
            backdrop_path: output.backdrop_path,
            first_air_date: output.first_air_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            genres: output.genres.into_iter().map(|g| Genre {id: g.id as i64,name: g.name,}).collect(),
            id: output.id as i64,
            number_of_episodes: output.number_of_episodes.map(|n| n as i64),
            number_of_seasons: output.number_of_seasons.map(|n| n as i64),
            original_title: output.original_title,
            overview: output.overview,
            poster_path: output.poster_path,
            season_summaries: output.season_summaries.into_iter().map(
                |s| TvSeasonSummary {
                    air_date: s.air_date.and_then(|a| NaiveDate::parse_from_str(&a, "%Y-%m-%d").ok()),
                    episode_count: s.episode_count as i64,
                    id: s.id as i64,
                    overview: s.overview,
                    poster_path: s.poster_path,
                    season_number: s.season_number as i64,
                    title: s.title,
                    vote_average: s.vote_average
                }
            ).collect(),
            tagline: output.tagline,
            title: output.title,
            vote_average: output.vote_average,
        }
    }
}
