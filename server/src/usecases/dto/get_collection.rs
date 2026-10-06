use uuid::Uuid;

use crate::usecases::dto::collection::Collection;

#[derive(Debug)]
pub(crate) struct GetCollectionInput {
    pub id: Uuid,
    pub user_id: Uuid
}

pub(crate) type GetCollectionOutput = Collection;
