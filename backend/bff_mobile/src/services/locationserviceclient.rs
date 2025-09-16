use async_trait::async_trait;
use tonic::transport::Channel;

use crate::services::location_service_client::LocationServiceClient;
use crate::services::{PostLocationReply, PostLocationRequest};

#[async_trait]
pub trait LocationServiceClientTrait: Send + Sync + 'static + Clone {
    async fn post_locations(
        &mut self,
        user_id: String,
        request: PostLocationRequest,
    ) -> Result<PostLocationReply, tonic::Status>;
}

#[async_trait]
impl LocationServiceClientTrait for LocationServiceClient<Channel> {
    async fn post_locations(
        &mut self,
        user_id: String,
        request: PostLocationRequest,
    ) -> Result<PostLocationReply, tonic::Status> {
        let mut request = tonic::Request::new(request);
        request.metadata_mut().insert("user_id", user_id.parse().unwrap());

        let response = self.post_location(request).await?;
        Ok(response.into_inner())
    }
}
