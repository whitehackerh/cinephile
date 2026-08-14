#[derive(Debug, Clone)]
pub(crate) struct TvEpisode {
    id: i32,
    episode_number: i32,
    season_number: i32,
    title: String,
    overview: Option<String>,
    runtime: Option<i32>,
    still_path: Option<String>,
    air_date: Option<String>,
    vote_average: Option<f64>,
    production_code: Option<String>,
    episode_type: Option<String>
}

impl TvEpisode{
    pub fn new(
        id: i32,
        episode_number: i32,
        season_number: i32,
        title: String,
        overview: Option<String>,
        runtime: Option<i32>,
        still_path: Option<String>,
        air_date: Option<String>,
        vote_average: Option<f64>,
        production_code: Option<String>,
        episode_type: Option<String>
    ) -> Self {
        Self {
            id,
            episode_number,
            season_number,
            title,
            overview,
            runtime,
            still_path,
            air_date,
            vote_average,
            production_code,
            episode_type
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn episode_number(&self) -> i32 {
        self.episode_number
    }

    pub fn season_number(&self) -> i32 {
        self.season_number
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

    pub fn still_path(&self) -> &Option<String> {
        &self.still_path
    }

    pub fn air_date(&self) -> &Option<String> {
        &self.air_date
    }

    pub fn vote_average(&self) -> Option<f64> {
        self.vote_average
    }

    pub fn production_code(&self) -> &Option<String> {
        &self.production_code
    }

    pub fn episode_type(&self) -> &Option<String> {
        &self.episode_type
    }

    pub fn into_parts(self) -> (
        i32, i32, i32, String, Option<String>, 
        Option<i32>, Option<String>, Option<String>, Option<f64>,
        Option<String>, Option<String> 
    ) {
        (
            self.id, self.episode_number, self.season_number, self.title, self.overview,
            self.runtime, self.still_path, self.air_date, self.vote_average,
            self.production_code, self.episode_type
        )
    }
}