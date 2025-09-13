use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::internal::domain::models::id::UserId;

pub trait JwtGenerator: Send + Sync + 'static {
    fn new(secret: &str) -> Self;
    fn new_jwt(&self, id: UserId) -> String;
    fn new_refresh_token(&self, id: UserId) -> String;
    fn verify_refresh_token(&self, token: &str) -> Option<UserId>;
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: UserId,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: UserId,
    pub exp: usize,
}

pub struct JwtGeneratorImpl {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtGenerator for JwtGeneratorImpl {
    fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        Self { encoding_key, decoding_key }
    }
    fn new_jwt(&self, id: UserId) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let claims = Claims { sub: id, exp: now + 15 * 60 };

        encode(&Header::default(), &claims, &self.encoding_key).unwrap()
    }

    fn new_refresh_token(&self, id: UserId) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let claims = RefreshClaims { sub: id, exp: now + 30 * 24 * 60 * 60 };

        encode(&Header::default(), &claims, &self.encoding_key).unwrap()
    }

    fn verify_refresh_token(&self, token: &str) -> Option<UserId> {
        let token_data =
            decode::<RefreshClaims>(token, &self.decoding_key, &Validation::default()).ok()?;
        Some(token_data.claims.sub)
    }
}
