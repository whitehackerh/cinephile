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