use axum::extract::State;
use axum::Json;

use crate::routes::response::{AppError, AppResult};
use crate::routes::state::AuthService;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::{RefreshLoginReply, RefreshLoginRequest};

#[utoipa::path(
    post,
    path = "/refresh_login",
    responses(
        (status = 200, description = "Refresh login successfully", body = RefreshLoginReply),
    ),
)]
pub async fn refresh_login<ASC>(
    State(AuthService(mut asc)): State<AuthService<ASC>>,
    Json(payload): Json<RefreshLoginRequest>,
) -> AppResult<Json<RefreshLoginReply>>
where
    ASC: AuthServiceClientTrait, {
    let request = RefreshLoginRequest { refreshtoken: payload.refreshtoken };

    let response = asc.refresh_login(request).await.map_err(AppError::internal_error)?;
    Ok(Json(response))
}
