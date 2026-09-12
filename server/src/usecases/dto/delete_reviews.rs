use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct DeleteReviewsInput {
    pub id: Uuid,
    pub user_id: Uuid,
}
