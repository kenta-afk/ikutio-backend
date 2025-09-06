use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST, };
use crate::domain::id::UserId;

pub struct AuthenticatedUser {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl AuthenticatedUser {
    pub fn new(name: String, email:String, password: String) -> Self {
        AuthenticatedUser {
            id: UserId(Uuid::new_v7()),
            name,
            email,
            password
        }
    }
    pub fn hash_password(&self, password: String) -> &str {
        let hashed_password = hash(password, DEFAULT_COST);
        hashed_password
    }
    pub fn verify_password(&self, password: String, hashed_password: String) -> bool {
        let result = verify(password, &hashed_password);
        result
    }
}