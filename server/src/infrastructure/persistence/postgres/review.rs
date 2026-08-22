use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::entities::review::Review;
use crate::usecases::repository::review::ReviewRepository;

pub(crate) struct PostgresReviewRepository {
    pool: PgPool,
}

impl PostgresReviewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for PostgresReviewRepository {
    async fn create(&self, review: &Review) -> Result<(), anyhow::Error> {
        // // sqlx::query!(
        // //     r#"
        // //     INSERT INTO users (id, name, email, password_hash)
        // //     VALUES ($1, $2, $3, $4)
        // //     "#,
        // //     user.id(),
        // //     user.name(),
        // //     user.email(),
        // //     user.password_hash(),
        // // )
        // .execute(&self.pool)
        // .await?;

        Ok(())
    }
}
