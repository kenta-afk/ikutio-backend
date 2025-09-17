use axum::Json;
use axum::extract::State;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::ProfileService;
use crate::services::profileserviceclient::ProfileServiceClientTrait;
use crate::services::{GetProfileReply, GetProfileRequest};

#[utoipa::path(
    post,
    path = "/get_profile",
    responses(
        (status = 200, description = "Get profile successfully", body = GetProfileReply),
    ),
)]

pub async fn get_profile<PSC>(
    authenticated_user: AuthenticatedUser,
    State(ProfileService(mut psc)): State<ProfileService<PSC>>,
) -> AppResult<Json<GetProfileReply>>
where
    PSC: ProfileServiceClientTrait, {
    let request = GetProfileRequest {};

    let response = psc
        .get_profile(request, authenticated_user.user_id)
        .await
        .map_err(AppError::internal_error)?;
    Ok(Json(response))
}
