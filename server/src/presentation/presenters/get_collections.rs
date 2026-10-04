use crate::{
    generated::api_schema::CollectionSummary,
    usecases::dto::get_collections::GetCollectionsOutput
};

pub struct GetCollectionsPresenter;

impl GetCollectionsPresenter {
    pub fn to_response(output: GetCollectionsOutput) -> Vec<CollectionSummary> {
        output.into_iter()
            .map(|c| CollectionSummary {
                cover_image_path: c.cover_image_path,
                created_at: c.created_at,
                description: c.description,
                id: c.id,
                title: c.title,
                updated_at: c.updated_at,
                work_target_paths: c.work_target_paths
            })
            .collect()
    }
}
