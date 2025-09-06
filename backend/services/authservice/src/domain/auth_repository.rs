pub trait AuthRepository: Send + Sync {
    pub fn save(&self, user: AuthenticatedUser) -> Result<(), AuthError>;
    pub fn find_by_email(&self, email: String) -> Option<AuthenticatedUser>;
}