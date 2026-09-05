use async_trait::async_trait;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::usecases::repository::{review::ReviewRepository, user::UserRepository};

pub struct TxRepositories {
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
    pub review_repo: Arc<dyn ReviewRepository + Send + Sync>,
}

pub type DynTxFuture = Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send>, anyhow::Error>> + Send>>;

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn execute_boxed(
        &self,
        f: Box<dyn FnOnce(Arc<TxRepositories>) -> DynTxFuture + Send>,
    ) -> Result<Box<dyn Any + Send>, anyhow::Error>;
}

#[async_trait]
pub trait UnitOfWorkExt: UnitOfWork {
    async fn execute<T, F, Fut>(&self, f: F) -> Result<T, anyhow::Error>
    where
        T: Send + 'static,
        Fut: Future<Output = Result<T, anyhow::Error>> + Send + 'static,
        F: FnOnce(Arc<TxRepositories>) -> Fut + Send + 'static,
    {
        let boxed_f = Box::new(move |repos: Arc<TxRepositories>| -> DynTxFuture {
            Box::pin(async move {
                let res = f(repos).await?;
                Ok(Box::new(res) as Box<dyn Any + Send>)
            })
        });

        let any_res = self.execute_boxed(boxed_f).await?;
        any_res
            .downcast::<T>()
            .map(|b| *b)
            .map_err(|_| anyhow::anyhow!("Failed to downcast transaction return type"))
    }
}

impl<U: UnitOfWork + ?Sized> UnitOfWorkExt for U {}