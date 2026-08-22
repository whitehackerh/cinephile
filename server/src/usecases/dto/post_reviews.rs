use serde::Deserialize;

use crate::usecases::dto::review::Review;


#[derive(Debug, Deserialize)]
pub(crate) struct PostReviewsInput {
    pub rating: i32,
    pub content: Option<String>,
    pub work_type: String,
    pub target_path: String
}

pub(crate) type PostReviewsOutput = Review;