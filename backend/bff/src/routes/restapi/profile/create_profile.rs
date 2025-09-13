use axum::Json;
use axum::extract::State;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::ProfileService;
use crate::services::profileserviceclient::ProfileServiceClientTrait;
use crate::services::{CreateProfileReply, CreateProfileRequest};

pub async fn create_profile<PSC>(
    _authenticated_user: AuthenticatedUser,
    State(ProfileService(mut psc)): State<ProfileService<PSC>>,
    Json(payload): Json<CreateProfileRequest>,
) -> AppResult<Json<CreateProfileReply>>
where
    PSC: ProfileServiceClientTrait, {
    let request = CreateProfileRequest { name: payload.name };

    let response = psc.create_profile(request).await.map_err(AppError::internal_error)?;
    Ok(Json(response))
}
