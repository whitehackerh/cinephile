use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        entities::collection::Collection,
        errors::AppError
    },
    usecases::{
        dto::{
            post_collections::{
                PostCollectionsInput,
                PostCollectionsOutput,
            },
            collection_work::CollectionWork
        },
        port::{
            post_collections::PostCollectionsUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        }
    }
};

pub(crate) struct PostCollectionsInteractor {
    uow: Arc<dyn UnitOfWork>
}

impl PostCollectionsInteractor {
    pub fn new(uow: Arc<dyn UnitOfWork>) -> Self {
        Self { uow }
    }
}

#[async_trait]
impl PostCollectionsUseCase for PostCollectionsInteractor {
    async fn execute(&self, input: PostCollectionsInput) -> Result<PostCollectionsOutput, AppError> {
        let collection = Collection::new(
            input.user_id,
            input.title,
            input.description,
        );

        self.uow.execute({
            let collection = collection.clone();
            move |repos| {
                Box::pin(async move {
                    repos.collection_repo.create(&collection).await?;
                    Ok(())
                })
            }
        })
        .await
        .map_err(|e| AppError::Infrastructure(e.to_string()))?;

        Ok(PostCollectionsOutput {
            id: collection.id(),
            title: collection.title().to_string(),
            description: collection.description().clone(),
            cover_image_path: collection.cover_image_path().clone(),
            works: collection.works()
                .into_iter()
                .map(|w| CollectionWork {
                    id: w.id(),
                    target_path: w.target_path().to_string(),
                    work_type: w.work_type().to_string(),
                    work: w.work().clone().into(),
                    added_at: w.added_at()
                })
                .collect(),
            created_at: collection.created_at(),
            updated_at: collection.updated_at(),
        })
    }
}
