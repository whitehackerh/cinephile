use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct GetReviewInput {
    pub user_id: Uuid,
    pub work_type: String,
    pub target_path: String
}

pub(crate) type GetReviewOutput = Review;
