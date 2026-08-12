#[derive(Debug, Clone)]
pub(crate) struct TvEpisodeSummary {
    id: i32,
    episode_number: i32,
    title: String,
    overview: Option<String>,
    runtime: Option<i32>,
    poster_path: Option<String>,
    air_date: Option<String>,
    vote_average: Option<f64>,
}

impl TvEpisodeSummary{
    pub fn new(
        id: i32,
        episode_number: i32,
        title: String,
        overview: Option<String>,
        runtime: Option<i32>,
        poster_path: Option<String>,
        air_date: Option<String>,
        vote_average: Option<f64>,
    ) -> Self {
        Self {
            id,
            episode_number,
            title,
            overview,
            runtime,
            poster_path,
            air_date,
            vote_average
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn episode_number(&self) -> i32 {
        self.episode_number
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn overview(&self) -> &Option<String> {
        &self.overview
    }

    pub fn runtime(&self) -> Option<i32> {
        self.runtime
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

    pub fn into_parts(self) -> (
        i32, i32, String, Option<String>, 
        Option<i32>, Option<String>, Option<String>, Option<f64>, 
    ) {
        (
            self.id, self.episode_number, self.title, self.overview,
            self.runtime, self.poster_path, self.air_date, self.vote_average
        )
    }
}