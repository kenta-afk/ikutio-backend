use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::LocationService;
use crate::services::GetLocationRequest;
use crate::services::locationserviceclient::LocationServiceClientTrait;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LocationGroup {
    pub location_id: String,
    pub locations: Vec<LocationData>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LocationsResponse {
    pub location_groups: Vec<LocationGroup>,
}

#[utoipa::path(
    get,
    path = "/get_locations",
    responses(
        (status = 200, description = "Get Locations successfully", body = LocationsResponse),
    ),
)]
pub async fn get_locations<LSC>(
    authenticated_user: AuthenticatedUser,
    State(LocationService(mut asc)): State<LocationService<LSC>>,
) -> AppResult<Json<LocationsResponse>>
where
    LSC: LocationServiceClientTrait, {
    let request = GetLocationRequest {};

    let response = asc
        .get_locations(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;

    // Parse the JSON string into location groups
    let location_groups: Vec<LocationGroup> =
        serde_json::from_str(&response.locations).map_err(|e| {
            AppError::internal_error(format!("Failed to parse location groups JSON: {e}"))
        })?;

    let locations_response = LocationsResponse { location_groups };
    Ok(Json(locations_response))
}
