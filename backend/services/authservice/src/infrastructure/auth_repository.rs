use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{auth_repository::AuthRepository, models::auth::AuthenticatedUser};

pub struct AuthRepositoryImpl{
    pool: PgPool,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn save(&self, user: AuthenticatedUser) -> Result<(), AuthError> {
        sqlx::query_as!(
            AuthenticatedUser,
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3, $4)",
            user.id,
            user.email,
            user.password
        )
    }
    async fn find_by_email(&self, email: &str) -> Option<AuthenticatedUser> {
        let authenticated_user = sqlx::query_as!(
            AuthenticatedUser,
            "SELECT id, email, password FROM users WHERE email = $1",
            email
        )
        .await
        .ok();

        Some(authenticated_user)
    }
}