use async_trait::async_trait;
use sqlx::PgPool;

use crate::internal::domain::auth_repository::AuthRepository;
use crate::internal::domain::models::auth::Auth;
use crate::internal::domain::models::id::UserId;
use crate::internal::infrastructure::error::DbError;

pub struct AuthRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn save(&self, user: &Auth) -> Result<(), DbError> {
        sqlx::query!(
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3)",
            user.id as _,
            user.email,
            user.password
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    async fn find_by_email(&self, email: &str) -> Option<Auth> {
        sqlx::query_as!(
            Auth,
            "SELECT id as \"id: UserId\", email, password FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }
}
