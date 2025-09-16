use axum::extract::FromRef;

use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::locationserviceclient::LocationServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

#[derive(Clone)]
pub struct AppState<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait,
> {
    pub asc: ASC,
    pub psc: PSC,
    pub gsc: GSC,
    pub lsc: LSC,
}

#[derive(Clone)]
pub struct AuthService<T>(pub T);

#[derive(Clone)]
pub struct ProfileService<T>(pub T);

#[derive(Clone)]
pub struct GameService<T>(pub T);

#[derive(Clone)]
pub struct LocationService<T>(pub T);

impl<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait,
> FromRef<AppState<ASC, PSC, GSC, LSC>> for AuthService<ASC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC, LSC>) -> Self {
        Self(state.asc.clone())
    }
}

impl<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait,
> FromRef<AppState<ASC, PSC, GSC, LSC>> for ProfileService<PSC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC, LSC>) -> Self {
        Self(state.psc.clone())
    }
}

impl<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait,
> FromRef<AppState<ASC, PSC, GSC, LSC>> for GameService<GSC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC, LSC>) -> Self {
        Self(state.gsc.clone())
    }
}
impl<
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait,
> FromRef<AppState<ASC, PSC, GSC, LSC>> for LocationService<LSC>
{
    fn from_ref(state: &AppState<ASC, PSC, GSC, LSC>) -> Self {
        Self(state.lsc.clone())
    }
}
