use uuid::Uuid;

use crate::usecases::dto::collection::Collection;

#[derive(Debug)]
pub(crate) struct PostCollectionsInput {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>
}

pub(crate) type PostCollectionsOutput = Collection;
