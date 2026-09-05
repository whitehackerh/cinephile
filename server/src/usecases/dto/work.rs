use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::work::Work as WorkEntity,
    usecases::dto::{
        movie::MovieOutput as Movie,
        tv_series::TvSeriesOutput as TvSeries,
        tv_season::TvSeasonOutput as TvSeason,
        tv_episode::TvEpisodeOutput as TvEpisode,
    }
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Work {
    Movie(Movie),
    TvSeries(TvSeries),
    TvSeason(TvSeason),
    TvEpisode(TvEpisode),
}

impl From<WorkEntity> for Work {
    fn from(entity: WorkEntity) -> Self {
        match entity {
            WorkEntity::Movie(m) => Work::Movie(m.into()),
            WorkEntity::TvSeries(s) => Work::TvSeries(s.into()),
            WorkEntity::TvSeason(s) => Work::TvSeason(s.into()),
            WorkEntity::TvEpisode(e) => Work::TvEpisode(e.into()),
        }
    }
}
