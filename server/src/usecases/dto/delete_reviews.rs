use uuid::Uuid;

use crate::usecases::dto::review::Review;


#[derive(Debug)]
pub(crate) struct DeleteReviewsInput {
    pub id: Uuid,
    pub user_id: Uuid,
}
