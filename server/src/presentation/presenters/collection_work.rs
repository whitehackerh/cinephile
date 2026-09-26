use crate::{
    generated::api_schema::{
        CollectionWork,
        CollectionWorkWork,
        CollectionWorkWorkType
    },
    presentation::presenters::{
        movie::MoviePresenter,
        tv_episode::TvEpisodePresenter,
        tv_season::TvSeasonPresenter,
        tv_series::TvSeriesPresenter
    },
    usecases::dto::{
        collection_work::CollectionWork as CollectionWorkOutput,
        work::Work
    }
};

pub struct CollectionWorkPresenter;

impl CollectionWorkPresenter {
    pub fn to_response(output: CollectionWorkOutput) -> CollectionWork {
        let work_type = CollectionWorkWorkType::try_from(output.work_type)
            .expect("work_type must be valid at presenter layer");

        let work = match output.work {
            Work::Movie(m) => CollectionWorkWork::Movie(MoviePresenter::to_response(m)),
            Work::TvSeries(s) => CollectionWorkWork::TvSeries(TvSeriesPresenter::to_response(s)),
            Work::TvSeason(sn) => CollectionWorkWork::TvSeason(TvSeasonPresenter::to_response(sn)),
            Work::TvEpisode(e) => CollectionWorkWork::TvEpisode(TvEpisodePresenter::to_response(e)),
        };

        CollectionWork {
            added_at: output.added_at,
            id: output.id,
            target_path: output.target_path,
            work: work,
            work_type: work_type
        }
    }
}
