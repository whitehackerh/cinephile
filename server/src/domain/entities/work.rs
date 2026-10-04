use crate::domain::entities::movie::Movie;
use crate::domain::entities::tv_series::TvSeries;
use crate::domain::entities::tv_season::TvSeason;
use crate::domain::entities::tv_episode::TvEpisode;

#[derive(Debug, Clone)]
pub enum Work {
    Movie(Movie),
    TvSeries(TvSeries),
    TvSeason(TvSeason),
    TvEpisode(TvEpisode),
}

impl Work {
    pub fn image_path(&self) -> Option<&str> {
        match self {
            Work::Movie(movie) => movie.poster_path().as_deref(),
            Work::TvSeries(series) => series.poster_path().as_deref(),
            Work::TvSeason(season) => season.poster_path().as_deref(),
            Work::TvEpisode(episode) => episode.still_path().as_deref(),
        }
    }
}