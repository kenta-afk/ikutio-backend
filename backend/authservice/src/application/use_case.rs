use tonic::{Request, Response, Status};

use crate::application::commands::login_command::LoginCommand;
use crate::application::commands::refresh_login_command::RefreshLoginCommand;
use crate::application::dtos::login_dto::LoginDto;
use crate::application::dtos::refresh_login_dto::RefreshLoginDto;
use crate::domain::auth_repository::AuthRepository;
use crate::domain::models::auth::AuthenticatedUser;
use crate::domain::models::error::AuthError;
use crate::infrastructure::jwt_generator::JwtGenerator;
use crate::infrastructure::uuid_generator::UuidGenerator;
use crate::proto::auth_service_server::AuthService;
use crate::proto::{LoginReply, LoginRequest, RefreshLoginReply, RefreshLoginRequest};

pub struct AuthServiceImpl<AR, UG, JG>
where
    AR: AuthRepository,
    UG: UuidGenerator,
    JG: JwtGenerator, {
    auth_repository: AR,
    uuid_generator: UG,
    jwt_generator: JG,
}

impl<AR, UG, JG> AuthServiceImpl<AR, UG, JG>
where
    AR: AuthRepository,
    UG: UuidGenerator,
    JG: JwtGenerator,
{
    pub fn new(auth_repository: AR, uuid_generator: UG, jwt_generator: JG) -> Self {
        AuthServiceImpl { auth_repository, uuid_generator, jwt_generator }
    }

    pub async fn login(&self, login_command: LoginCommand) -> Result<LoginDto, AuthError> {
        let email = login_command.email;
        let password = login_command.password;

        let user = match self.auth_repository.find_by_email(&email).await {
            Some(existing_user) => {
                if !existing_user.verify_password(password, &existing_user.password) {
                    return Err(AuthError::InvalidPassword);
                }
                existing_user
            }
            None => {
                let new_user = AuthenticatedUser::new(email, password, &self.uuid_generator);

                let hashed_password = new_user.hash_password(&new_user.password)?;

                let new_user = AuthenticatedUser { password: hashed_password, ..new_user };
                self.auth_repository.save(&new_user).await?;
                new_user
            }
        };

        let jwt = self.jwt_generator.new_jwt(user.id);
        let refresh_token = self.jwt_generator.new_refresh_token(user.id);

        Ok(LoginDto { token: jwt, refresh_token, id: user.id })
    }

    pub async fn refresh_login(
        &self,
        refreshlogin_command: RefreshLoginCommand,
    ) -> Result<RefreshLoginDto, AuthError> {
        let refresh_token = refreshlogin_command.refresh_token;

        let user_id = match self.jwt_generator.verify_refresh_token(&refresh_token) {
            Some(id) => id,
            None => return Err(AuthError::InvalidPassword),
        };

        let new_jwt = self.jwt_generator.new_jwt(user_id);
        let new_refresh_token = self.jwt_generator.new_refresh_token(user_id);

        Ok(RefreshLoginDto { jwt: new_jwt, refresh_token: new_refresh_token })
    }
}

#[tonic::async_trait]
impl<AR, UG, JG> AuthService for AuthServiceImpl<AR, UG, JG>
where
    AR: AuthRepository,
    UG: UuidGenerator,
    JG: JwtGenerator,
{
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        match AuthServiceImpl::login(self, request.into_inner().into()).await {
            Ok(login_dto) => Ok(Response::new(login_dto.into())),
            Err(auth_error) => Err(auth_error.into()),
        }
    }

    async fn refresh_login(
        &self,
        request: Request<RefreshLoginRequest>,
    ) -> Result<Response<RefreshLoginReply>, Status> {
        match AuthServiceImpl::refresh_login(self, request.into_inner().into()).await {
            Ok(refreshlogin_dto) => Ok(Response::new(refreshlogin_dto.into())),
            Err(auth_error) => Err(auth_error.into()),
        }
    }
}
