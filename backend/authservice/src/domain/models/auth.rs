use bcrypt::{DEFAULT_COST, hash, verify};

use crate::domain::models::error::AuthError;
use crate::domain::models::id::UserId;
use crate::infrastructure::uuid_generator::UuidGenerator;

pub struct AuthenticatedUser {
    pub id: UserId,
    pub email: String,
    pub password: String,
}

impl AuthenticatedUser {
    pub fn new(email: String, password: String, generator: &impl UuidGenerator) -> Self {
        AuthenticatedUser { id: UserId::new(generator), email, password }
    }
    pub fn hash_password(&self, password: &String) -> Result<String, AuthError> {
        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(AuthError::FailedHashError),
        };
        Ok(hashed_password)
    }
    pub fn verify_password(&self, password: String, hashed_password: &String) -> bool {
        let result = match verify(password, &hashed_password) {
            Ok(result) => result,
            Err(_) => return false,
        };
        result
    }
}
