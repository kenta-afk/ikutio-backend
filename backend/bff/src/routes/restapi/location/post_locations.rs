use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::LocationService;
use crate::services::locationserviceclient::LocationServiceClientTrait;
use crate::services::{PostLocationReply, PostLocationRequest};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostLocationPayload {
    #[serde(rename = "pathData")]
    pub path_data: Vec<LocationData>,
}

pub async fn post_locations<LSC>(
    authenticated_user: AuthenticatedUser,
    State(LocationService(mut lsc)): State<LocationService<LSC>>,
    Json(payload): Json<PostLocationPayload>,
) -> AppResult<Json<PostLocationReply>>
where
    LSC: LocationServiceClientTrait, {
    let locations_json = serde_json::to_string(&payload.path_data)
        .map_err(|e| AppError::internal_error(format!("Failed to serialize locations: {e}")))?;

    let request = PostLocationRequest { locations: locations_json };

    let response = lsc
        .post_locations(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;

    Ok(Json(response))
}
