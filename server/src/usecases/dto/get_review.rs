use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct GetReviewInput {
    pub id: Uuid,
    pub user_id: Uuid
}

pub(crate) type GetReviewOutput = Review;
