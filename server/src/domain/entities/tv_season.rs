use crate::domain::entities::tv_episode_summary::TvEpisodeSummary;

#[derive(Debug, Clone)]
pub(crate) struct TvSeason {
    id: i32,
    season_number: i32,
    episode_count: i32,
    title: String,
    overview: Option<String>,
    poster_path: Option<String>,
    air_date: Option<String>,
    vote_average: Option<f64>,
    episode_summaries: Vec<TvEpisodeSummary>
}

impl TvSeason{
    pub fn new(
        id: i32,
        season_number: i32,
        episode_count: i32,
        title: String,
        overview: Option<String>,
        poster_path: Option<String>,
        air_date: Option<String>,
        vote_average: Option<f64>,
        episode_summaries: Vec<TvEpisodeSummary>
    ) -> Self {
        Self {
            id,
            season_number,
            episode_count,
            title,
            overview,
            poster_path,
            air_date,
            vote_average,
            episode_summaries
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn season_number(&self) -> i32 {
        self.season_number
    }

    pub fn episode_count(&self) -> i32 {
        self.episode_count
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn overview(&self) -> &Option<String> {
        &self.overview
    }

    pub fn poster_path(&self) -> &Option<String> {
        &self.poster_path
    }

    pub fn air_date(&self) -> &Option<String> {
        &self.air_date
    }

    pub fn vote_average(&self) -> Option<f64> {
        self.vote_average
    }

    pub fn episode_summaries(&self) -> &Vec<TvEpisodeSummary> {
        &self.episode_summaries
    }

    pub fn into_parts(self) -> (
        i32, i32, i32, String, Option<String>,
        Option<String>, Option<String>, Option<f64>, 
        Vec<TvEpisodeSummary>
    ) {
        (
            self.id, self.season_number, self.episode_count, self.title, self.overview,
            self.poster_path, self.air_date, self.vote_average,
            self.episode_summaries
        )
    }
}