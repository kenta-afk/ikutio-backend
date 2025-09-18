use async_trait::async_trait;
use sqlx::PgPool;

use crate::internal::domain::models::auth::Auth;
use crate::internal::infrastructure::error::DbError;

#[async_trait]
pub trait AuthRepository: Send + Sync + 'static {
    fn new(pool: PgPool) -> Self;
    async fn save(&self, user: &Auth) -> Result<(), DbError>;
    async fn find_by_email(&self, email: &str) -> Option<Auth>;
}
