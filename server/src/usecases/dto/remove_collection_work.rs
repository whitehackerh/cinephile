use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct RemoveCollectionWorkInput {
    pub id: Uuid,
    pub collection_work_id: Uuid,
    pub user_id: Uuid
}
