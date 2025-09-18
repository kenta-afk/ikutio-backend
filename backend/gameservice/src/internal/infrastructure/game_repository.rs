use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::internal::domain::game_repository::GameRepository;
use crate::internal::domain::models::game::Game;
use crate::internal::infrastructure::error::DbError;

pub struct GameRepositoryImpl {
    client: Client,
}

#[async_trait]
impl GameRepository for GameRepositoryImpl {
    fn new(client: Client) -> Self {
        Self { client }
    }
    async fn save(&self, game: Game) -> Result<(), DbError> {
        let mut item = HashMap::new();
        item.insert("game_id".to_string(), AttributeValue::S(game.game_id.to_string()));
        item.insert("user_id".to_string(), AttributeValue::S(game.user_id.to_string()));
        item.insert("is_finished".to_string(), AttributeValue::Bool(game.is_finished));
        item.insert("score".to_string(), AttributeValue::N(game.score.to_string()));

        self.client.put_item().table_name("games").set_item(Some(item)).send().await?;

        Ok(())
    }
}
