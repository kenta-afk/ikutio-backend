use crate::services::authserviceclient::AuthServiceClientTrait;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::post,
};
use crate::routes::restapi::auth::login::login;

use tower_http::cors::CorsLayer;
use crate::routes::state::AppState;

pub fn create_routes<ASC>(asc: ASC) -> Router
where
    ASC: AuthServiceClientTrait,
{
    let state = AppState {
        asc,
    };
    
    Router::new()
        .route("/login", post(login))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([
                    Method::POST,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_credentials(true)
        )
}

