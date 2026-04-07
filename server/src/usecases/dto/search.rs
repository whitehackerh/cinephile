use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct SearchInput {
    pub query: String,
    pub page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchOutput {
    pub works: Vec<Work>,
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "media_type")]
pub(crate) enum Work {
    #[serde(rename = "movie")]
    Movie(MovieSummary),
    #[serde(rename = "tv")]
    Tv(TvSummary),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MovieSummary {
    pub id: i32,
    pub title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSummary {
    pub id: i32,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
}
