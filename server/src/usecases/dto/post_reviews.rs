use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct PostReviewsInput {
    pub user_id: Uuid,
    pub rating: i32,
    pub content: Option<String>,
    pub work_type: String,
    pub target_path: String
}

pub(crate) type PostReviewsOutput = Review;
