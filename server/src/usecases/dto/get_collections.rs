use uuid::Uuid;

use crate::usecases::dto::collection::CollectionWithoutWork;


#[derive(Debug)]
pub(crate) struct GetCollectionsInput {
    pub user_id: Uuid,
}

pub(crate) type GetCollectionsOutput = Vec<CollectionWithoutWork>;
