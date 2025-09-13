use axum::extract::FromRef;

use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

#[derive(Clone)]
pub struct AppState<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait> {
    pub asc: ASC,
    pub psc: PSC,
}

#[derive(Clone)]
pub struct AuthService<T>(pub T);

#[derive(Clone)]
pub struct ProfileService<T>(pub T);

impl<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait> FromRef<AppState<ASC, PSC>>
    for AuthService<ASC>
{
    fn from_ref(state: &AppState<ASC, PSC>) -> Self {
        Self(state.asc.clone())
    }
}

impl<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait> FromRef<AppState<ASC, PSC>>
    for ProfileService<PSC>
{
    fn from_ref(state: &AppState<ASC, PSC>) -> Self {
        Self(state.psc.clone())
    }
}
