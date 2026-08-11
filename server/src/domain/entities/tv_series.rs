use crate::domain::entities::{
    genre::Genre,
    tv_season_summary::TvSeasonSummary,
};

#[derive(Debug, Clone)]
pub(crate) struct TvSeries {
    id: i32,
    title: String,
    original_title: String,
    overview: Option<String>,
    number_of_seasons: Option<i32>,
    number_of_episodes: Option<i32>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    first_air_date: Option<String>,
    vote_average: Option<f64>,
    tagline: Option<String>,
    genres: Vec<Genre>,
    season_summaries: Vec<TvSeasonSummary>
}

impl TvSeries{
    pub fn new(
        id: i32,
        title: String,
        original_title: String,
        overview: Option<String>,
        number_of_seasons: Option<i32>,
        number_of_episodes: Option<i32>,
        poster_path: Option<String>,
        backdrop_path: Option<String>,
        first_air_date: Option<String>,
        vote_average: Option<f64>,
        tagline: Option<String>,
        genres: Vec<Genre>,
        season_summaries: Vec<TvSeasonSummary>
    ) -> Self {
        Self {
            id,
            title,
            original_title,
            overview,
            number_of_seasons,
            number_of_episodes,
            poster_path,
            backdrop_path,
            first_air_date,
            vote_average,
            tagline,
            genres,
            season_summaries
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn original_title(&self) -> &str {
        &self.original_title
    }

    pub fn overview(&self) -> &Option<String> {
        &self.overview
    }

    pub fn number_of_seasons(&self) -> Option<i32> {
        self.number_of_seasons
    }

    pub fn number_of_episodes(&self) -> Option<i32> {
        self.number_of_episodes
    }

    pub fn poster_path(&self) -> &Option<String> {
        &self.poster_path
    }

    pub fn backdrop_path(&self) -> &Option<String> {
        &self.backdrop_path
    }

    pub fn first_air_date(&self) -> &Option<String> {
        &self.first_air_date
    }

    pub fn vote_average(&self) -> Option<f64> {
        self.vote_average
    }

    pub fn tagline(&self) -> &Option<String> {
        &self.tagline
    }

    pub fn genres(&self) -> &Vec<Genre> {
        &self.genres
    }

    pub fn season_summaries(&self) -> &Vec<TvSeasonSummary> {
        &self.season_summaries
    }

    pub fn into_parts(self) -> (
        i32, String, String, Option<String>, Option<i32>, Option<i32>,
        Option<String>, Option<String>, Option<String>, Option<f64>, 
        Option<String>, Vec<Genre>, Vec<TvSeasonSummary>
    ) {
        (
            self.id, self.title, self.original_title, self.overview, self.number_of_seasons, self.number_of_episodes,
            self.poster_path, self.backdrop_path, self.first_air_date,
            self.vote_average, self.tagline, self.genres, self.season_summaries
        )
    }
}