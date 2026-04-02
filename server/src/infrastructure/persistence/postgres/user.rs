use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::entities::user::User;
use crate::usecases::repository::user::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, email, password_hash FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            User::reconstruct(
                r.id,
                r.name,
                r.email,
                r.password_hash,
            )
        }))
    }

    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
            user.id(),
            user.name(),
            user.email(),
            user.password_hash(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
