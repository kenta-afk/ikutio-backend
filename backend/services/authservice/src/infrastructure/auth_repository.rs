use crate::domain::auth_repository::AuthRepository;

pub struct SqlxAuthRepository {
    auth_repository: AuthRepository,
}