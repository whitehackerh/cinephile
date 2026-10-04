use crate::{
    generated::api_schema::Collection,
    presentation::presenters::collection::CollectionPresenter,
    usecases::dto::collection::Collection as CollectionOutput,
};

pub struct CreateCollectionPresenter;

impl CreateCollectionPresenter {
    pub fn to_response(output: CollectionOutput) -> Collection {
        CollectionPresenter::to_response(output)
    }
}
