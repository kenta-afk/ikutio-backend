use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::models::auth::AuthenticatedUser;
use crate::domain::models::error::AuthError;

#[async_trait]
pub trait AuthRepository: Send + Sync + 'static {
    fn new(pool: PgPool) -> Self;
    async fn save(&self, user: &AuthenticatedUser) -> Result<(), AuthError>;
    async fn find_by_email(&self, email: &str) -> Option<AuthenticatedUser>;
}
