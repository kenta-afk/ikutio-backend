use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationGroup {
    pub location_id: String,
    pub locations: Vec<Location>,
}

use crate::internal::domain::location_repository::LocationRepository;
use crate::internal::domain::models::error::LocationError;
use crate::internal::domain::models::id::{LocationId, UserId};
use crate::internal::domain::models::location::{Location, Locations};
use crate::internal::infrastructure::uuid_generator::UuidGenerator;
use crate::proto::location_service_server::LocationService;
use crate::proto::{GetLocationReply, GetLocationRequest, PostLocationReply, PostLocationRequest};

pub struct LocationServiceImpl<LR, UG>
where
    LR: LocationRepository,
    UG: UuidGenerator, {
    location_repository: LR,
    uuid_generator: UG,
}

impl<LR, UG> LocationServiceImpl<LR, UG>
where
    LR: LocationRepository,
    UG: UuidGenerator,
{
    pub fn new(location_repository: LR, uuid_generator: UG) -> Self {
        Self { location_repository, uuid_generator }
    }

    pub async fn save_locations(
        &self,
        request: Request<PostLocationRequest>,
    ) -> Result<PostLocationReply, LocationError> {
        // gRPCのmetadataからuser_idを取得
        let metadata = request.metadata();
        let user_id_str = metadata
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or(LocationError::InvalidInput)?;

        let user_uuid =
            uuid::Uuid::parse_str(user_id_str).map_err(|_| LocationError::InvalidInput)?;
        let user_id = UserId::from_uuid(user_uuid);
        let location_id = LocationId::new(&self.uuid_generator);

        let request = request.into_inner();
        let locations_json = request.locations;

        // JSON文字列をVec<Location>に変換
        let locations_vec: Vec<crate::internal::domain::models::location::Location> =
            serde_json::from_str(&locations_json).map_err(|_| LocationError::InvalidInput)?;

        let locations = Locations::new(location_id, user_id, locations_vec, false);
        self.location_repository.save(locations).await?;
        Ok(PostLocationReply {})
    }

    pub async fn get_locations(
        &self,
        request: Request<GetLocationRequest>,
    ) -> Result<GetLocationReply, LocationError> {
        // gRPCのmetadataからuser_idを取得
        let metadata = request.metadata();
        let user_id_str = metadata
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or(LocationError::InvalidInput)?;

        let user_uuid =
            uuid::Uuid::parse_str(user_id_str).map_err(|_| LocationError::InvalidInput)?;
        let user_id = UserId::from_uuid(user_uuid);

        let locations_list = self.location_repository.get(user_id).await?;

        // location_id毎にグループ化
        let mut location_groups = Vec::new();
        for locations in locations_list {
            let group = LocationGroup {
                location_id: locations.location_id.to_string(),
                locations: locations.locations,
            };
            location_groups.push(group);
        }

        // LocationGroupsをJSON文字列に変換
        let locations_json = serde_json::to_string(&location_groups)
            .map_err(|e| LocationError::InternalError(e.to_string()))?;

        Ok(GetLocationReply { locations: locations_json })
    }
}

#[tonic::async_trait]
impl<LR, UG> LocationService for LocationServiceImpl<LR, UG>
where
    LR: LocationRepository,
    UG: UuidGenerator,
{
    async fn post_location(
        &self,
        request: Request<PostLocationRequest>,
    ) -> Result<Response<PostLocationReply>, Status> {
        match self.save_locations(request).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(err) => Err(err.into()),
        }
    }

    async fn get_location(
        &self,
        request: Request<GetLocationRequest>,
    ) -> Result<Response<GetLocationReply>, Status> {
        match self.get_locations(request).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(err) => Err(err.into()),
        }
    }
}
