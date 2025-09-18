use bcrypt::{DEFAULT_COST, hash, verify};

use crate::internal::domain::models::error::AuthError;
use crate::internal::domain::models::id::UserId;
use crate::internal::infrastructure::uuid_generator::UuidGenerator;

pub struct Auth {
    pub id: UserId,
    pub email: String,
    pub password: String,
}

impl Auth {
    pub fn new(email: String, password: String, generator: &impl UuidGenerator) -> Self {
        Auth { id: UserId::new(generator), email, password }
    }
    pub fn hash_password(&self, password: &String) -> Result<String, AuthError> {
        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(AuthError::FailedHashError),
        };
        Ok(hashed_password)
    }
    pub fn verify_password(&self, password: String, hashed_password: &String) -> bool {
        verify(password, hashed_password).unwrap_or_default()
    }
}
