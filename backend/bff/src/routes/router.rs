use axum::Router;
use axum::http::{HeaderValue, Method, header};
use axum::routing::post;
use tower_http::cors::CorsLayer;

use crate::routes::restapi::auth::login::login;
use crate::routes::restapi::auth::refresh_login::refresh_login;
use crate::routes::restapi::game::start_game::start_game;
use crate::routes::restapi::profile::create_profile::create_profile;
use crate::routes::state::AppState;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

pub fn create_routes<ASC, PSC, GSC>(asc: ASC, psc: PSC, gsc: GSC) -> Router
where
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait, {
    let state = AppState { asc, psc, gsc };

    Router::new()
        .route("/login", post(login))
        .route("/refresh_login", post(refresh_login))
        .route("/create_profile", post(create_profile))
        .route("/start_game", post(start_game))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([Method::POST])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}
