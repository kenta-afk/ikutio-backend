use axum::Router;
use axum::http::{HeaderValue, Method, header};
use axum::routing::post;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::restapi::auth::login::login;
use crate::routes::restapi::auth::refresh_login::refresh_login;
use crate::routes::restapi::location::post_locations::post_locations;
use crate::routes::state::AppState;
use crate::services::authserviceclient::AuthServiceClientTrait;
use crate::services::locationserviceclient::LocationServiceClientTrait;

pub fn create_routes<ASC, LSC>(asc: ASC, lsc: LSC) -> Router
where
    ASC: AuthServiceClientTrait,
    LSC: LocationServiceClientTrait, {
    let state = AppState { asc, lsc };

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(login))
        .route("/refresh_login", post(refresh_login))
        .route("/post_locations", post(post_locations))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([Method::POST, Method::GET])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::restapi::auth::login::login,
    crate::routes::restapi::auth::refresh_login::refresh_login,
    crate::routes::restapi::location::post_locations::post_locations,
))]
struct ApiDoc;
