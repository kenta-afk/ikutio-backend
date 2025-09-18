use async_trait::async_trait;
use tonic::transport::Channel;
use tonic::Streaming;

use crate::services::game_service_client::GameServiceClient;
use crate::services::{StartGameReply, StartGameRequest};

#[async_trait]
pub trait GameServiceClientTrait: Send + Sync + 'static + Clone {
    async fn start_game(
        &mut self,
        user_id: String,
        request: StartGameRequest,
    ) -> Result<Streaming<StartGameReply>, tonic::Status>;
}

#[async_trait]
impl GameServiceClientTrait for GameServiceClient<Channel> {
    async fn start_game(
        &mut self,
        user_id: String,
        request: StartGameRequest,
    ) -> Result<Streaming<StartGameReply>, tonic::Status> {
        let mut request = tonic::Request::new(request);
        request.metadata_mut().insert("user_id", user_id.parse().unwrap());

        let response = self.start_game(request).await?;
        Ok(response.into_inner())
    }
}
