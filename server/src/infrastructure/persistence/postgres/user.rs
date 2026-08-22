use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::entities::user::User;
use crate::usecases::repository::user::UserRepository;
use super::executor::PgConn;

#[derive(Clone)]
pub struct PostgresUserRepository {
    conn: PgConn,
}

impl PostgresUserRepository {
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
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let query = sqlx::query!(
            r#"
            SELECT id, name, email, password_hash FROM users WHERE email = $1
            "#,
            email
        );

        let row = match &self.conn {
            PgConn::Pool(pool) => query.fetch_optional(pool).await?,
            PgConn::Tx(tx) => {
                let mut guard = tx.lock().await;
                query.fetch_optional(&mut **guard).await?
            }
        };

        Ok(row.map(|r| {
            User::reconstruct(
                r.id,
                r.name,
                r.email,
                r.password_hash,
            )
        }))
    }

    async fn create(&self, user: &User) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
            user.id(),
            user.name(),
            user.email(),
            user.password_hash(),
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
