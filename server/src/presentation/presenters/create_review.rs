use crate::{
    generated::api_schema::Review,
    presentation::presenters::review::ReviewPresenter,
    usecases::dto::review::Review as ReviewOutput,
};

pub struct CreateReviewPresenter;

impl CreateReviewPresenter {
    pub fn to_response(output: ReviewOutput) -> Review {
        ReviewPresenter::to_response(output)
    }
}
