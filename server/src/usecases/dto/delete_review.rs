use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct DeleteReviewInput {
    pub id: Uuid,
    pub user_id: Uuid,
}
