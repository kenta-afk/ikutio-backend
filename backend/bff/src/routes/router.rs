use axum::Router;
use axum::http::{HeaderValue, Method, header};
use axum::routing::post;
use tower_http::cors::CorsLayer;

use crate::routes::restapi::auth::login::login;
use crate::routes::restapi::auth::refresh_login::refresh_login;
use crate::routes::restapi::profile::create_profile::create_profile;
use crate::routes::state::AppState;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

pub fn create_routes<ASC, PSC>(asc: ASC, psc: PSC) -> Router
where
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait, {
    let state = AppState { asc, psc };

    Router::new()
        .route("/login", post(login))
        .route("/refresh_login", post(refresh_login))
        .route("/create_profile", post(create_profile))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([Method::POST])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}
