use axum::Json;
use axum::extract::State;

use crate::routes::response::{AppError, AppResult};
use crate::routes::state::AuthService;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::{LoginReply, LoginRequest};
pub async fn login<ASC>(
    State(AuthService(mut asc)): State<AuthService<ASC>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginReply>>
where
    ASC: AuthServiceClientTrait, {
    let request = LoginRequest { email: payload.email, password: payload.password };

    let response = asc.login(request).await.map_err(AppError::internal_error)?;
    Ok(Json(response))
}
