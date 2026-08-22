use chrono::NaiveDate;

use crate::{
    generated::api_schema::{
        Genre,
        Movie,
    },
    usecases::dto::movie::MovieOutput
};

pub struct MoviePresenter;

impl MoviePresenter {
    pub fn to_response(output: MovieOutput) -> Movie {
        Movie {
            backdrop_path: output.backdrop_path,
            genres: output.genres.into_iter().map(|g| Genre {id: g.id as i64,name: g.name,}).collect(),
            id: output.id as i64,
            original_title: output.original_title,
            overview: output.overview,
            poster_path: output.poster_path,
            release_date: output.release_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            runtime: output.runtime.map(|r| r as i64),
            tagline: output.tagline,
            title: output.title,
            vote_average: output.vote_average,
        }
    }
}
