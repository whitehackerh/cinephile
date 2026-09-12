use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct GetReviewsInput {
    pub user_id: Uuid,
}

pub(crate) type GetReviewsOutput = Vec<Review>;
