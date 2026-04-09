use crate::domain::entities::genre::Genre;

#[derive(Debug, Clone)]
pub(crate) struct Movie {
    id: i32,
    title: String,
    original_title: String,
    over_view: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    release_date: Option<String>,
    runtime: Option<i32>,
    vote_average: Option<f64>,
    tagline: Option<String>,
    genres: Vec<Genre>,
}

impl Movie{
    pub fn new(
        id: i32,
        title: String,
        original_title: String,
        over_view: Option<String>,
        poster_path: Option<String>,
        backdrop_path: Option<String>,
        release_date: Option<String>,
        runtime: Option<i32>,
        vote_average: Option<f64>,
        tagline: Option<String>,
        genres: Vec<Genre>,
    ) -> Self {
        Self {
            id,
            title,
            original_title,
            over_view,
            poster_path,
            backdrop_path,
            release_date,
            runtime,
            vote_average,
            tagline,
            genres,
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

    pub fn over_view(&self) -> &Option<String> {
        &self.over_view
    }

    pub fn poster_path(&self) -> &Option<String> {
        &self.poster_path
    }

    pub fn backdrop_path(&self) -> &Option<String> {
        &self.backdrop_path
    }

    pub fn release_date(&self) -> &Option<String> {
        &self.release_date
    }

    pub fn runtime(&self) -> Option<i32> {
        self.runtime
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
}
