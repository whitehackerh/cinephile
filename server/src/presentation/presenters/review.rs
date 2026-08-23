use crate::{
    generated::api_schema::{
        Review,
        ReviewWork,
        ReviewWorkType
    },
    presentation::presenters::{
        movie::MoviePresenter,
        tv_episode::TvEpisodePresenter,
        tv_season::TvSeasonPresenter,
        tv_series::TvSeriesPresenter
    },
    usecases::dto::{
        review::Review as ReviewOutput,
        work::Work
    }
};

pub struct ReviewPresenter;

impl ReviewPresenter {
    pub fn to_response(output: ReviewOutput) -> Review {
        let work_type = ReviewWorkType::try_from(output.work_type)
                    .expect("work_type must be valid at presenter layer");

        let work = match output.work {
            Work::Movie(m) => ReviewWork::Movie(MoviePresenter::to_response(m)),
            Work::TvSeries(s) => ReviewWork::TvSeries(TvSeriesPresenter::to_response(s)),
            Work::TvSeason(sn) => ReviewWork::TvSeason(TvSeasonPresenter::to_response(sn)),
            Work::TvEpisode(e) => ReviewWork::TvEpisode(TvEpisodePresenter::to_response(e)),
        };

        Review {
            content: output.content,
            created_at: output.created_at,
            deleted_at: output.deleted_at,
            id: output.id,
            rating: output.rating as i64,
            target_path: output.target_path,
            updated_at: output.updated_at,
            work: work,
            work_type: work_type,
        }
    }
}
