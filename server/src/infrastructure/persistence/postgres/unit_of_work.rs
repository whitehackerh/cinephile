use async_trait::async_trait;
use sqlx::PgPool;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::usecases::port::unit_of_work::{DynTxFuture, TxRepositories, UnitOfWork};
use super::{review::PostgresReviewRepository, user::PostgresUserRepository};

pub struct PostgresUnitOfWork {
    pool: PgPool,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    async fn execute_boxed(
        &self,
        f: Box<dyn FnOnce(Arc<TxRepositories>) -> DynTxFuture + Send>,
    ) -> Result<Box<dyn Any + Send>, anyhow::Error> {
        let tx = self.pool.begin().await?;
        let tx_shared = Arc::new(Mutex::new(tx));

        let repos = Arc::new(TxRepositories {
            user_repo: Arc::new(PostgresUserRepository::new_tx(tx_shared.clone())),
            review_repo: Arc::new(PostgresReviewRepository::new_tx(tx_shared.clone())),
        });

        let result = f(repos).await;

        match result {
            Ok(val) => {
                let tx = Arc::try_unwrap(tx_shared)
                    .map_err(|_| anyhow::anyhow!("Failed to unwrap Tx lock (references still exist)"))?
                    .into_inner();
                tx.commit().await?;
                Ok(val)
            }
            Err(err) => {
                let tx = Arc::try_unwrap(tx_shared)
                    .map_err(|_| anyhow::anyhow!("Failed to unwrap Tx lock (references still exist)"))?
                    .into_inner();
                let _ = tx.rollback().await;
                Err(err)
            }
        }
    }
}
