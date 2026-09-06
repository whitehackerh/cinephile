use crate::{
    generated::api_schema::Review,
    presentation::presenters::review::ReviewPresenter,
    usecases::dto::review::Review as ReviewOutput,
};

pub struct GetReviewPresenter;

impl GetReviewPresenter {
    pub fn to_response(output: Option<ReviewOutput>) -> Option<Review> {
        output.map(ReviewPresenter::to_response)
    }
}
