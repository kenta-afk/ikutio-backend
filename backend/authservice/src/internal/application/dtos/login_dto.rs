use crate::domain::models::id::UserId;

pub struct LoginDto {
    pub token: String,
    pub refresh_token: String,
    pub id: UserId,
}
