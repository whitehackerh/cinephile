use chrono::NaiveDate;
use std::num::NonZeroU64;
use crate::{
    generated::api_schema::{
        MovieSummary,
        MovieSummaryMediaType,
        SearchResponse,
        TvSummary,
        TvSummaryMediaType,
        Work
    },
    usecases::dto::search::{
        SearchOutput,
        Work as WorkDto
    }
};

pub struct SearchPresenter;

impl SearchPresenter {
    pub fn to_response(output: SearchOutput) -> SearchResponse {
        SearchResponse {
            page: NonZeroU64::new(output.page as u64).unwrap_or(NonZeroU64::MIN),
            total_pages: NonZeroU64::new(output.total_pages as u64).unwrap_or(NonZeroU64::MIN),
            total_results: output.total_results as u64,
            works: output.works.into_iter().map(Work::from).collect(),
        }
    }
}

impl From<WorkDto> for Work {
    fn from(work: WorkDto) -> Self {
        match work {
            WorkDto::Movie(m) => Work::MovieSummary(MovieSummary {
                id: m.id as i64,
                media_type: MovieSummaryMediaType::Movie,
                title: m.title,
                overview: m.overview,
                poster_path: m.poster_path,
                release_date: m.release_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            }),
            WorkDto::Tv(t) => Work::TvSummary(TvSummary {
                id: t.id as i64,
                media_type: TvSummaryMediaType::Tv,
                name: t.name,
                overview: t.overview,
                poster_path: t.poster_path,
                first_air_date: t.first_air_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            })
        }
    }
}