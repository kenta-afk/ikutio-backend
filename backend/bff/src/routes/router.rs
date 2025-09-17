use axum::Router;
use axum::http::{HeaderName, HeaderValue, Method, header};
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::restapi::auth::login::login;
use crate::routes::restapi::auth::refresh_login::refresh_login;
use crate::routes::restapi::game::start_game::start_game;
use crate::routes::restapi::location::get_locations::get_locations;
use crate::routes::restapi::profile::create_profile::create_profile;
use crate::routes::restapi::profile::get_profile::get_profile;
use crate::routes::state::AppState;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::locationserviceclient::LocationServiceClientTrait;
use crate::services::profileserviceclient::ProfileServiceClientTrait;

pub fn create_routes<ASC, PSC, GSC, LSC>(asc: ASC, psc: PSC, gsc: GSC, lsc: LSC) -> Router
where
    ASC: AuthServiceClientTrait,
    PSC: ProfileServiceClientTrait,
    GSC: GameServiceClientTrait,
    LSC: LocationServiceClientTrait, {
    let state = AppState { asc, psc, gsc, lsc };

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(login))
        .route("/refresh_login", post(refresh_login))
        .route("/create_profile", post(create_profile))
        .route("/get_profile", get(get_profile))
        .route("/start_game", get(start_game))
        .route("/get_locations", get(get_locations))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([Method::POST, Method::GET])
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::CONNECTION,
                    header::UPGRADE,
                    HeaderName::from_static("sec-websocket-key"),
                    HeaderName::from_static("sec-websocket-version"),
                ]),
        )
}
#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::restapi::auth::login::login,
    crate::routes::restapi::auth::refresh_login::refresh_login,
    crate::routes::restapi::profile::create_profile::create_profile,
    crate::routes::restapi::profile::get_profile::get_profile,
    crate::routes::restapi::game::start_game::start_game,
    crate::routes::restapi::location::get_locations::get_locations,
))]
struct ApiDoc;
