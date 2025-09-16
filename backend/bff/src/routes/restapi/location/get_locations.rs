use axum::Json;
use axum::extract::State;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::LocationService;
use crate::services::locationserviceclient::LocationServiceClientTrait;
use crate::services::{GetLocationReply, GetLocationRequest};

pub async fn get_locations<LSC>(
    authenticated_user: AuthenticatedUser,
    State(LocationService(mut asc)): State<LocationService<LSC>>,
) -> AppResult<Json<GetLocationReply>>
where
    LSC: LocationServiceClientTrait, {
    let request = GetLocationRequest {};

    let response = asc
        .get_locations(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;
    Ok(Json(response))
}
