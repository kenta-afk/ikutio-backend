use async_trait::async_trait;
use tonic::transport::Channel;

use crate::services::auth_service_client::AuthServiceClient;
use crate::services::{LoginReply, LoginRequest, RefreshLoginReply, RefreshLoginRequest};

#[async_trait]
pub trait AuthServiceClientTrait: Send + Sync + 'static + Clone {
    async fn login(&mut self, request: LoginRequest) -> Result<LoginReply, tonic::Status>;
    async fn refresh_login(
        &mut self,
        reqeust: RefreshLoginRequest,
    ) -> Result<RefreshLoginReply, tonic::Status>;
}

#[async_trait]
impl AuthServiceClientTrait for AuthServiceClient<Channel> {
    async fn login(&mut self, request: LoginRequest) -> Result<LoginReply, tonic::Status> {
        let response = self.login(request).await?;
        Ok(response.into_inner())
    }
    async fn refresh_login(
        &mut self,
        request: RefreshLoginRequest,
    ) -> Result<RefreshLoginReply, tonic::Status> {
        let response = self.refresh_login(request).await?;
        Ok(response.into_inner())
    }
}
