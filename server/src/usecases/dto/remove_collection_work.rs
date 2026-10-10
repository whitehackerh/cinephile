use uuid::Uuid;

use crate::usecases::dto::collection::Collection;

#[derive(Debug)]
pub(crate) struct RemoveCollectionWorkInput {
    pub id: Uuid,
    pub collection_work_id: Uuid,
    pub user_id: Uuid
}

pub(crate) type RemoveCollectionWorkOutput = Collection;
