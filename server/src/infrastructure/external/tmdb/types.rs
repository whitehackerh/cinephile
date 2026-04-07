use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TmdbSearchResponse {
    pub page: u32,
    pub results: Vec<TmdbMedia>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "media_type")]
pub enum TmdbMedia {
    #[serde(rename = "movie")]
    Movie(TmdbMovie),
    #[serde(rename = "tv")]
    Tv(TmdbTv),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct TmdbMovie {
    pub id: i32,
    pub title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbTv {
    pub id: i32,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub first_air_date: Option<String>,
}
