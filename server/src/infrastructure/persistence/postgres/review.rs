use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::{
    domain::entities::review::Review,
    infrastructure::persistence::postgres::executor::PgConn,
    usecases::{
        dto::review::ReviewWithoutWork,
        repository::review::ReviewRepository
    }
};

pub(crate) struct PostgresReviewRepository {
    conn: PgConn,
}

impl PostgresReviewRepository {
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
impl ReviewRepository for PostgresReviewRepository {
    async fn exists_by_user_and_target(&self, user_id: &Uuid, target_path: &str) -> anyhow::Result<bool> {
        let query = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM reviews
                WHERE user_id = $1 
                  AND target_path = $2
                  AND deleted_at IS NULL
            ) AS "exists!"
            "#,
            user_id,
            target_path
        );

        let exists = match &self.conn {
            PgConn::Pool(pool) => query.fetch_one(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_one(&mut **guard).await?
            }
        };

        Ok(exists)
    }

    async fn create(&self, review: &Review) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO reviews (
                id, user_id, rating, content, work_type,
                target_path, created_at, updated_at, deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            review.id(),
            review.user_id(),
            review.rating() as i16,
            review.content().as_deref(),
            review.work_type(),
            review.target_path(),
            review.created_at(),
            review.updated_at(),
            review.deleted_at()
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

    async fn find_by_work_type_target_path(&self, user_id: &Uuid, work_type: &str, target_path: &str) -> anyhow::Result<Option<ReviewWithoutWork>> {
        let query = sqlx::query_as!(
            ReviewWithoutWork,
            r#"
            SELECT
                id,
                rating AS "rating: i32",
                content,
                created_at,
                updated_at,
                deleted_at
            FROM reviews
            WHERE user_id = $1 AND work_type = $2 AND target_path = $3 AND deleted_at IS NULL
            "#,
            user_id, work_type, target_path
        );

        let review_without_work = match &self.conn {
            PgConn::Pool(pool) => query.fetch_optional(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_optional(&mut **guard).await?
            }
        };

        Ok(review_without_work)
    }
}
