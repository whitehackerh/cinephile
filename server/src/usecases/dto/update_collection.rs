use uuid::Uuid;

use crate::usecases::dto::collection::Collection;

#[derive(Debug)]
pub(crate) struct UpdateCollectionInput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>
}

pub(crate) type UpdateCollectionOutput = Collection;
