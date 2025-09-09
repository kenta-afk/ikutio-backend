use axum::extract::FromRef;
use crate::services::authserviceclient::AuthServiceClientTrait;

#[derive(Clone)]
pub struct AppState<ASC: AuthServiceClientTrait> {
    pub asc: ASC,
}

#[derive(Clone)]
pub struct AuthService<T>(pub T);

impl<ASC: AuthServiceClientTrait> FromRef<AppState<ASC>> for AuthService<ASC> {
    fn from_ref(state: &AppState<ASC>) -> Self {
        Self(state.asc.clone())
    }
}