use async_trait::async_trait;
use tonic::transport::Channel;

use crate::services::auth_service_client::AuthServiceClient;
use crate::services::{LoginReply, LoginRequest};

#[async_trait]
pub trait AuthServiceClientTrait: Send + Sync + 'static + Clone {
    async fn login(&mut self, request: LoginRequest) -> Result<LoginReply, tonic::Status>;
}

#[async_trait]
impl AuthServiceClientTrait for AuthServiceClient<Channel> {
    async fn login(&mut self, request: LoginRequest) -> Result<LoginReply, tonic::Status> {
        let response = self.login(tonic::Request::new(request)).await?;
        Ok(response.into_inner())
    }
}
