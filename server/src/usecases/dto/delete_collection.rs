use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct DeleteCollectionInput {
    pub id: Uuid,
    pub user_id: Uuid,
}
