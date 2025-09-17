use async_trait::async_trait;
use aws_sdk_dynamodb::Client;

use crate::internal::domain::models::id::UserId;
use crate::internal::domain::models::location::Locations;
use crate::internal::infrastructure::error::DbError;

#[async_trait]
pub trait LocationRepository: Send + Sync + 'static {
    fn new(client: Client) -> Self;
    async fn save(&self, locations: Locations) -> Result<(), DbError>;
    async fn get(&self, user_id: UserId) -> Result<Vec<Locations>, DbError>;
}
