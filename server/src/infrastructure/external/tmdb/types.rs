use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbSearchResponse {
    pub page: u32,
    pub results: Vec<TmdbMedia>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "media_type")]
pub enum TmdbMedia {
    #[serde(rename = "movie")]
    Movie(TmdbMovieSummary),
    #[serde(rename = "tv")]
    Tv(TmdbTvSummary),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbMovieSummary {
    pub id: i32,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbTvSummary {
    pub id: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub first_air_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbMovie {
    pub id: i32,
    pub title: String,
    pub original_title: String,
    #[serde(default)]
    pub over_view: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    #[serde(default)]
    pub vote_average: Option<f32>,
    #[serde(default)]
    pub tagline: Option<String>,
    pub genres: Vec<TmdbGenre>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbGenre {
    pub id: i32,
    pub name: String,
}