use axum::extract::State;

use crate::{
    routes::{
        state::AuthService, 
        response::{AppResult, AppError}
    }, 
    services::{authserviceclient::AuthServiceClientTrait, LoginRequest, LoginReply}
};

use axum::Json;
pub async fn login<ASC>(
    State(AuthService(mut asc)): State<AuthService<ASC>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginReply>>
where
    ASC: AuthServiceClientTrait,
{
    let request = LoginRequest {
        email: payload.email,
        password: payload.password,
    };

    let response = asc.login(request).await
        .map_err(|e| AppError::internal_error(e))?;
    Ok(Json(response))
}