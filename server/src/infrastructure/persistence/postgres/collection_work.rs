use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::entities::collection_work::CollectionWork,
    infrastructure::persistence::postgres::executor::PgConn,
    usecases::{
        repository::collection_work::CollectionWorkRepository
    }
};

pub(crate) struct PostgresCollectionWorkRepository {
    conn: PgConn,
}

impl PostgresCollectionWorkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            conn: PgConn::Pool(pool),
        }
    }

    pub fn new_tx(tx: Arc<Mutex<Transaction<'static, Postgres>>>) -> Self {
        Self {
            conn: PgConn::Tx(tx),
        }
    }
}

#[async_trait]
impl CollectionWorkRepository for PostgresCollectionWorkRepository {
    async fn create(&self, work: &CollectionWork, collection_id: &Uuid) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO collection_works (
                id, collection_id, work_type, target_path, added_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            work.id(),
            collection_id,
            work.work_type(),
            work.target_path(),
            work.added_at()
        );

        match &self.conn {
            PgConn::Pool(pool) => {
                query.execute(pool).await?;
            }
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.execute(&mut **guard).await?;
            }
        };

        Ok(())
    }

    async fn delete(&self, collection_work_id: &Uuid) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            DELETE FROM collection_works
            WHERE id = $1
            "#,
            collection_work_id,
        );

        match &self.conn {
            PgConn::Pool(pool) => {
                query.execute(pool).await?;
            }
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.execute(&mut **guard).await?;
            }
        };

        Ok(())
    }
}