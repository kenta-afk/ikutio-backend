use axum::Json;
use axum::extract::State;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::LocationService;
use crate::services::locationserviceclient::LocationServiceClientTrait;
use crate::services::{PostLocationReply, PostLocationRequest};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationDataInternal {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PostLocationPayload {
    #[serde(rename = "pathData")]
    pub path_data: Vec<LocationData>,
}

#[utoipa::path(
    post,
    path = "/post_locations",
    responses(
        (status = 200, description = "Post locations successfully", body = PostLocationReply),
    ),
)]
pub async fn post_locations<LSC>(
    authenticated_user: AuthenticatedUser,
    State(LocationService(mut asc)): State<LocationService<LSC>>,
    Json(payload): Json<PostLocationPayload>,
) -> AppResult<Json<PostLocationReply>>
where
    LSC: LocationServiceClientTrait, {
    // Convert timestamps from string to i64
    let converted_locations: Result<Vec<LocationDataInternal>, _> = payload
        .path_data
        .into_iter()
        .map(|loc| {
            let timestamp_i64 = DateTime::parse_from_rfc3339(&loc.timestamp)
                .map_err(|e| format!("Failed to parse timestamp {}: {}", loc.timestamp, e))?
                .timestamp();

            Ok(LocationDataInternal {
                latitude: loc.latitude,
                longitude: loc.longitude,
                timestamp: timestamp_i64,
            })
        })
        .collect();

    let converted_locations =
        converted_locations.map_err(|e: String| AppError::internal_error(e))?;

    let locations_json = serde_json::to_string(&converted_locations)
        .map_err(|e| AppError::internal_error(format!("Failed to serialize locations: {e}")))?;

    let request = PostLocationRequest { locations: locations_json };

    let response = asc
        .post_locations(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;

    Ok(Json(response))
}
