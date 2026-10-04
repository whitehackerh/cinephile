use uuid::Uuid;
use crate::usecases::dto::collection::CollectionSummary;

#[derive(Debug)]
pub(crate) struct GetCollectionsInput {
    pub user_id: Uuid,
}

pub(crate) type GetCollectionsOutput = Vec<CollectionSummary>;
