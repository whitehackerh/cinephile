use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct PatchReviewsInput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub content: Option<String>,
}

pub(crate) type PatchReviewsOutput = Review;
