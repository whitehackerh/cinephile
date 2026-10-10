use uuid::Uuid;

use crate::usecases::dto::collection::Collection;

#[derive(Debug)]
pub(crate) struct AddCollectionWorkInput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub work_type: String,
    pub target_path: String
}

pub(crate) type AddCollectionWorkOutput = Collection;
