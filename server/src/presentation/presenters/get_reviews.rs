use crate::{
    generated::api_schema::Review,
    presentation::presenters::review::ReviewPresenter,
    usecases::dto::review::Review as ReviewOutput,
};

pub struct GetReviewsPresenter;

impl GetReviewsPresenter {
    pub fn to_response(output: Vec<ReviewOutput>) -> Vec<Review> {
        output.into_iter()
            .map(ReviewPresenter::to_response)
            .collect()
    }
}
