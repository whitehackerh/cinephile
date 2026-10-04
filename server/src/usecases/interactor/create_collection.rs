use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        entities::collection::Collection,
        errors::AppError
    },
    usecases::{
        dto::{
            create_collection::{
                CreateCollectionInput,
                CreateCollectionOutput,
            },
            collection_work::CollectionWork
        },
        port::{
            create_collection::CreateCollectionUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        }
    }
};

pub(crate) struct CreateCollectionInteractor {
    uow: Arc<dyn UnitOfWork>
}

impl CreateCollectionInteractor {
    pub fn new(uow: Arc<dyn UnitOfWork>) -> Self {
        Self { uow }
    }
}

#[async_trait]
impl CreateCollectionUseCase for CreateCollectionInteractor {
    async fn execute(&self, input: CreateCollectionInput) -> Result<CreateCollectionOutput, AppError> {
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

        Ok(CreateCollectionOutput {
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
