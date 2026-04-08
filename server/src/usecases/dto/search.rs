use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct SearchInput {
    pub query: String,
    pub page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchOutput {
    pub works: Vec<Work>,
    pub page: u32,
    pub total_pages: u32,
    pub total_results: u32,
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
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TvSummary {
    pub id: i32,
    pub name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
}
