use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{
    domain::entities::collection::Collection,
    infrastructure::persistence::postgres::executor::PgConn,
    usecases::repository::collection::CollectionRepository
};

pub(crate) struct PostgresCollectionRepository {
    conn: PgConn,
}

impl PostgresCollectionRepository {
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
impl CollectionRepository for PostgresCollectionRepository {
    async fn create(&self, collection: &Collection) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO collections (
                id, user_id, title, description,
                cover_image_path, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            collection.id(),
            collection.user_id(),
            collection.title(),
            collection.description().as_deref(),
            collection.cover_image_path().as_deref(),
            collection.created_at(),
            collection.updated_at()
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
