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
pub(crate) enum TmdbMedia {
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
    pub overview: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub runtime: Option<i32>,
    #[serde(default)]
    pub vote_average: Option<f64>,
    #[serde(default)]
    pub tagline: Option<String>,
    #[serde(default)]
    pub genres: Option<Vec<TmdbGenre>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbGenre {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbTvSeries {
    pub id: i32,
    pub name: String,
    pub original_name: String,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(default)]
    pub number_of_seasons: Option<i32>,
    #[serde(default)]
    pub number_of_episodes: Option<i32>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub first_air_date: Option<String>,
    #[serde(default)]
    pub vote_average: Option<f64>,
    #[serde(default)]
    pub tagline: Option<String>,
    #[serde(default)]
    pub genres: Option<Vec<TmdbGenre>>,
    #[serde(default)]
    pub seasons: Option<Vec<TmdbTvSeasonSummary>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TmdbTvSeasonSummary {
    pub id: i32,
    #[serde(default)]
    pub season_number: i32,
    #[serde(default)]
    pub episode_count: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub air_date: Option<String>,
    #[serde(default)]
    pub vote_average: Option<f64>
}
