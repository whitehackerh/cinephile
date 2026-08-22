// use serde::{Deserialize, Serialize};

// use crate::{
//     presentation::presenters::{
//         movie::MovieResponse as Movie,
//         tv_series::TvSeriesResponse as TvSeries,
//         tv_season::TvSeasonResponse as TvSeason,
//         tv_episode::TvEpisodeResponse as TvEpisode,
//     },
//     usecases::dto::work::Work as WorkOutput
// };

// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum Work {
//     Movie(Movie),
//     TvSeries(TvSeries),
//     TvSeason(TvSeason),
//     TvEpisode(TvEpisode),
// }

// impl From<WorkOutput> for Work {
//     fn from(output: WorkOutput) -> Self {
//         match output {
//             WorkOutput::Movie(m) => Work::Movie(Movie::from(m)),
//             WorkOutput::TvSeries(s) => Work::TvSeries(TvSeries::from(s)),
//             WorkOutput::TvSeason(s) => Work::TvSeason(TvSeason::from(s)),
//             WorkOutput::TvEpisode(e) => Work::TvEpisode(TvEpisode::from(e)),
//         }
//     }
// }