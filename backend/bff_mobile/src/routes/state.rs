use axum::extract::FromRef;

use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::locationserviceclient::LocationServiceClientTrait;

#[derive(Clone)]
pub struct AppState<ASC: AuthServiceClientTrait, LSC: LocationServiceClientTrait> {
    pub asc: ASC,
    pub lsc: LSC,
}

#[derive(Clone)]
pub struct AuthService<T>(pub T);

#[derive(Clone)]
pub struct LocationService<T>(pub T);

impl<ASC: AuthServiceClientTrait, LSC: LocationServiceClientTrait> FromRef<AppState<ASC, LSC>>
    for AuthService<ASC>
{
    fn from_ref(state: &AppState<ASC, LSC>) -> Self {
        Self(state.asc.clone())
    }
}

impl<ASC: AuthServiceClientTrait, LSC: LocationServiceClientTrait> FromRef<AppState<ASC, LSC>>
    for LocationService<LSC>
{
    fn from_ref(state: &AppState<ASC, LSC>) -> Self {
        Self(state.lsc.clone())
    }
}
