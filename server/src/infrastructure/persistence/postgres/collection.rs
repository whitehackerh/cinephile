use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::{
    domain::entities::collection::Collection,
    infrastructure::persistence::postgres::executor::PgConn,
    usecases::{
        dto::{
            collection_work::CollectionWorkForReconstruct,
            collection::{
                CollectionForReconstruct,
                CollectionSummary,
                CollectionWithoutWork
            }
        },
        repository::collection::CollectionRepository
    }
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

    async fn find_by_id_with_works(&self, id: &Uuid, user_id: &Uuid) -> anyhow::Result<Option<CollectionForReconstruct>> {
        let query = sqlx::query!(
            r#"
            SELECT
                c.id AS collection_id,
                c.title,
                c.description,
                c.cover_image_path,
                c.created_at,
                c.updated_at,
                cw.id AS "work_id?",
                cw.target_path AS "target_path?",
                cw.work_type AS "work_type?",
                cw.added_at AS "added_at?"
            FROM collections c
            LEFT JOIN collection_works cw ON c.id = cw.collection_id
            WHERE c.id = $1 AND c.user_id = $2
            "#,
            id, user_id
        );

        let rows = match &self.conn {
            PgConn::Pool(pool) => query.fetch_all(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_all(&mut **guard).await?
            }
        };

        if rows.is_empty() {
            return Ok(None);
        }

        let collection_id = rows[0].collection_id;
        let title = rows[0].title.clone();
        let description = rows[0].description.clone();
        let cover_image_path = rows[0].cover_image_path.clone();
        let created_at = rows[0].created_at;
        let updated_at = rows[0].updated_at;

        let works = rows
            .into_iter()
            .filter_map(|row| {
                match (row.work_id, row.target_path, row.work_type, row.added_at) {
                    (Some(id), Some(target_path), Some(work_type), Some(added_at)) => {
                        Some(CollectionWorkForReconstruct {
                            id,
                            target_path,
                            work_type,
                            added_at,
                        })
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(Some(CollectionForReconstruct {
            id: collection_id,
            title,
            description,
            cover_image_path,
            works,
            created_at,
            updated_at,
        }))
    }

    async fn find_by_id_with_works_for_update(&self, id: &Uuid, user_id: &Uuid) -> anyhow::Result<Option<CollectionForReconstruct>> {
        let query = sqlx::query!(
            r#"
            SELECT
                c.id AS collection_id,
                c.title,
                c.description,
                c.cover_image_path,
                c.created_at,
                c.updated_at,
                cw.id AS "work_id?",
                cw.target_path AS "target_path?",
                cw.work_type AS "work_type?",
                cw.added_at AS "added_at?"
            FROM collections c
            LEFT JOIN collection_works cw ON c.id = cw.collection_id
            WHERE c.id = $1 AND c.user_id = $2
            FOR UPDATE OF c
            "#,
            id, user_id
        );

        let rows = match &self.conn {
            PgConn::Pool(pool) => query.fetch_all(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_all(&mut **guard).await?
            }
        };

        if rows.is_empty() {
            return Ok(None);
        }

        let collection_id = rows[0].collection_id;
        let title = rows[0].title.clone();
        let description = rows[0].description.clone();
        let cover_image_path = rows[0].cover_image_path.clone();
        let created_at = rows[0].created_at;
        let updated_at = rows[0].updated_at;

        let works = rows
            .into_iter()
            .filter_map(|row| {
                match (row.work_id, row.target_path, row.work_type, row.added_at) {
                    (Some(id), Some(target_path), Some(work_type), Some(added_at)) => {
                        Some(CollectionWorkForReconstruct {
                            id,
                            target_path,
                            work_type,
                            added_at,
                        })
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(Some(CollectionForReconstruct {
            id: collection_id,
            title,
            description,
            cover_image_path,
            works,
            created_at,
            updated_at,
        }))
    }

    async fn update(&self, collection: &Collection) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            UPDATE collections
            SET
                title = $1,
                description = $2,
                cover_image_path = $3,
                updated_at = $4
            WHERE id = $5 AND user_id = $6
            "#,
            collection.title(),
            collection.description().as_deref(),
            collection.cover_image_path().as_deref(),
            collection.updated_at(),
            collection.id(),
            collection.user_id()
        );

        match &self.conn {
            PgConn::Pool(pool) => { query.execute(pool).await?; },
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.execute(&mut **guard).await?;
            }
        };

        Ok(())
    }

    async fn fetch_all(&self, user_id: &Uuid) -> anyhow::Result<Vec<CollectionWithoutWork>> {
        let query = sqlx::query_as!(
            CollectionWithoutWork,
            r#"
            SELECT
                id,
                title,
                description,
                cover_image_path,
                created_at,
                updated_at
            FROM collections
            WHERE user_id = $1
            ORDER BY updated_at DESC
            "#,
            user_id
        );

        let collection_without_work_list = match &self.conn {
            PgConn::Pool(pool) => query.fetch_all(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_all(&mut **guard).await?
            }
        };

        Ok(collection_without_work_list)
    }

    async fn fetch_all_with_target_paths(
        &self,
        user_id: &Uuid,
    ) -> anyhow::Result<Vec<CollectionSummary>> {
        let query = sqlx::query!(
            r#"
            SELECT
                c.id AS collection_id,
                c.title,
                c.description,
                c.cover_image_path,
                c.created_at,
                c.updated_at,
                cw.target_path AS "target_path?"
            FROM collections c
            LEFT JOIN collection_works cw ON c.id = cw.collection_id
            WHERE c.user_id = $1
            ORDER BY c.created_at DESC, c.id DESC, cw.added_at ASC
            "#,
            user_id
        );

        let rows = match &self.conn {
            PgConn::Pool(pool) => query.fetch_all(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_all(&mut **guard).await?
            }
        };

        let mut summaries: Vec<CollectionSummary> = Vec::new();

        for row in rows {
            if let Some(last) = summaries.last_mut() {
                if last.id == row.collection_id {
                    if let Some(target_path) = row.target_path {
                        last.work_target_paths.push(target_path);
                    }
                    continue;
                }
            }

            let mut work_target_paths = Vec::new();
            if let Some(target_path) = row.target_path {
                work_target_paths.push(target_path);
            }

            summaries.push(CollectionSummary {
                id: row.collection_id,
                title: row.title,
                description: row.description,
                cover_image_path: row.cover_image_path,
                work_target_paths,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        Ok(summaries)
    }

    async fn delete(&self, collection: &Collection) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            DELETE FROM collections
            WHERE id = $1 AND user_id = $2
            "#,
            collection.id(),
            collection.user_id()
        );

        match &self.conn {
            PgConn::Pool(pool) => { query.execute(pool).await?; },
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.execute(&mut **guard).await?;
            }
        };

        Ok(())
    }
}
