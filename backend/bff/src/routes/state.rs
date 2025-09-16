use axum::extract::FromRef;

use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

#[derive(Clone)]
pub struct AppState<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
> {
    pub asc: ASC,
    pub psc: PSC,
    pub gsc: GSC,
}

#[derive(Clone)]
pub struct AuthService<T>(pub T);

#[derive(Clone)]
pub struct ProfileService<T>(pub T);

#[derive(Clone)]
pub struct GameService<T>(pub T);

impl<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait, GSC: GameServiceClientTrait>
    FromRef<AppState<ASC, PSC, GSC>> for AuthService<ASC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC>) -> Self {
        Self(state.asc.clone())
    }
}

impl<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait, GSC: GameServiceClientTrait>
    FromRef<AppState<ASC, PSC, GSC>> for ProfileService<PSC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC>) -> Self {
        Self(state.psc.clone())
    }
}

impl<ASC: AuthServiceClientTrait, PSC: ProfileServiceClientTrait, GSC: GameServiceClientTrait>
    FromRef<AppState<ASC, PSC, GSC>> for GameService<GSC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC>) -> Self {
        Self(state.gsc.clone())
    }
}
