use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::models::auth::AuthenticatedUser;
use crate::infrastructure::error::DbError;

#[async_trait]
pub trait AuthRepository: Send + Sync + 'static {
    fn new(pool: PgPool) -> Self;
    async fn save(&self, user: &AuthenticatedUser) -> Result<(), DbError>;
    async fn find_by_email(&self, email: &str) -> Option<AuthenticatedUser>;
}
