use crate::{
    generated::api_schema::Collection,
    presentation::presenters::collection_work::CollectionWorkPresenter,
    usecases::dto::collection::Collection as CollectionOutput
};

pub struct CollectionPresenter;

impl CollectionPresenter {
    pub fn to_response(output: CollectionOutput) -> Collection {
        Collection {
            cover_image_path: output.cover_image_path,
            created_at: output.created_at,
            description: output.description,
            id: output.id,
            title: output.title,
            updated_at: output.updated_at,
            works: output.works.into_iter().map(CollectionWorkPresenter::to_response).collect(),
        }
    }
}
