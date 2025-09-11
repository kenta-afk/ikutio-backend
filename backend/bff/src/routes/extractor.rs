use std::env;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Debug, Clone)]
struct AuthenticatedUser {
    user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header =
            parts.headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) => {
                if let Some(user_id) = extract_user_id_from_token(auth_header) {
                    Ok(AuthenticatedUser { user_id })
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

fn extract_user_id_from_token(auth_header: &str) -> Option<String> {
    let token = auth_header.strip_prefix("Bearer ")?;
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &decoding_key, &validation).ok()?;

    Some(token_data.claims.sub)
}
