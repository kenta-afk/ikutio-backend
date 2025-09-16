use async_trait::async_trait;
use aws_sdk_dynamodb::Client;

use crate::internal::domain::models::game::Game;
use crate::internal::infrastructure::error::DbError;

#[async_trait]
pub trait GameRepository: Send + Sync + 'static {
    fn new(client: Client) -> Self;
    async fn save(&self, game: Game) -> Result<(), DbError>;
}
