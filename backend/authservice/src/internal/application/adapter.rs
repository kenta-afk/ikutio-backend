use std::sync::Arc;

use tonic::Status;

use crate::internal::application::commands::login_command::LoginCommand;
use crate::internal::application::commands::refresh_login_command::RefreshLoginCommand;
use crate::internal::application::dtos::login_dto::LoginDto;
use crate::internal::application::dtos::refresh_login_dto::RefreshLoginDto;
use crate::internal::application::use_case::AuthServiceImpl;
use crate::internal::domain::auth_repository::AuthRepository;
use crate::internal::domain::models::error::AuthError;
use crate::internal::infrastructure::error::DbError;
use crate::internal::infrastructure::jwt_generator::JwtGenerator;
use crate::internal::infrastructure::uuid_generator::UuidGenerator;
use crate::proto::{LoginReply, LoginRequest, RefreshLoginReply, RefreshLoginRequest};

impl From<LoginRequest> for LoginCommand {
    fn from(request: LoginRequest) -> Self {
        LoginCommand { email: request.email, password: request.password }
    }
}

impl From<LoginDto> for LoginReply {
    fn from(dto: LoginDto) -> Self {
        LoginReply { jwt: dto.token, refresh_token: dto.refresh_token, id: dto.id.to_string() }
    }
}

impl From<RefreshLoginRequest> for RefreshLoginCommand {
    fn from(request: RefreshLoginRequest) -> Self {
        RefreshLoginCommand { refresh_token: request.refreshtoken }
    }
}

impl From<RefreshLoginDto> for RefreshLoginReply {
    fn from(dto: RefreshLoginDto) -> Self {
        RefreshLoginReply { jwt: dto.jwt, refreshtoken: dto.refresh_token }
    }
}

// AuthErrorからtonicのStatusへの変換
impl From<AuthError> for Status {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidPassword => Status::unauthenticated("Invalid password"),
            AuthError::FailedHashError => Status::internal("Internal server error"),
            AuthError::UserNotFound => Status::not_found("User not found"),
            AuthError::DatabaseError => Status::internal("Database error"),
            AuthError::ConfigurationError => Status::internal("Configuration error"),
            AuthError::InternalError => Status::internal("Internal server error"),
        }
    }
}

// DbErrorからAuthErrorへの変換
impl From<DbError> for AuthError {
    fn from(error: DbError) -> Self {
        match error {
            // ユーザーが見つからない場合（find_by_email等で使用）
            DbError::RowNotFound => AuthError::UserNotFound,

            // 設定関連のエラー
            DbError::Configuration(_) => AuthError::ConfigurationError,

            // データベース固有のエラー
            DbError::Database(_) => AuthError::DatabaseError,

            // エンコード/デコード関連のエラー
            DbError::Encode(_) => AuthError::InternalError,
            DbError::Decode(_) => AuthError::InternalError,
            DbError::ColumnDecode { .. } => AuthError::InternalError,

            // その他の内部エラー
            DbError::InvalidArgument(_) => AuthError::InternalError,
            DbError::Io(_) => AuthError::InternalError,
            DbError::TypeNotFound { .. } => AuthError::InternalError,
            DbError::ColumnIndexOutOfBounds { .. } => AuthError::InternalError,
            DbError::ColumnNotFound(_) => AuthError::InternalError,
            DbError::AnyDriverError(_) => AuthError::InternalError,
            DbError::WorkerCrashed => AuthError::InternalError,
        }
    }
}

pub struct AuthServiceGrpcAdapter<AR, UG, JG>
where
    AR: AuthRepository + Send + Sync + 'static,
    UG: UuidGenerator + Send + Sync + 'static,
    JG: JwtGenerator + Send + Sync + 'static, {
    auth_service: Arc<AuthServiceImpl<AR, UG, JG>>,
}

impl<AR, UG, JG> AuthServiceGrpcAdapter<AR, UG, JG>
where
    AR: AuthRepository + Send + Sync + 'static,
    UG: UuidGenerator + Send + Sync + 'static,
    JG: JwtGenerator + Send + Sync + 'static,
{
    pub fn new(auth_service: AuthServiceImpl<AR, UG, JG>) -> Self {
        Self { auth_service: Arc::new(auth_service) }
    }
}
