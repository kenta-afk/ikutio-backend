use axum::Router;
use axum::http::{HeaderValue, Method, header};
use axum::routing::post;
use tower_http::cors::CorsLayer;

use crate::routes::restapi::auth::login::login;
use crate::routes::state::AppState;
use crate::services::authserviceclient::AuthServiceClientTrait;

pub fn create_routes<ASC>(asc: ASC) -> Router
where
    ASC: AuthServiceClientTrait, {
    let state = AppState { asc };

    Router::new().route("/login", post(login)).with_state(state).layer(
        CorsLayer::new()
            .allow_origin(HeaderValue::from_static("*"))
            .allow_methods([Method::POST])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_credentials(true),
    )
}
