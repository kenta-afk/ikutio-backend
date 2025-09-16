use tonic::Status;

use crate::internal::application::commands::start_game_command::StartGameCommand;
use crate::internal::domain::models::error::GameError;
use crate::internal::infrastructure::error::DbError;
use crate::proto::StartGameRequest;

impl From<StartGameRequest> for StartGameCommand {
    fn from(_request: StartGameRequest) -> Self {
        StartGameCommand {}
    }
}

impl From<GameError> for Status {
    fn from(error: GameError) -> Self {
        match error {
            GameError::AlreadyExists => Status::already_exists("Record already exists"),
            GameError::NotFoundRecord => Status::not_found("Record not found"),
            GameError::InvalidInput => Status::invalid_argument("Invalid input"),
            GameError::InternalError(_) => Status::internal("Internal server error"),
        }
    }
}

impl From<DbError> for GameError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Conflict(_) => GameError::AlreadyExists,
            DbError::NotFound(_) => GameError::NotFoundRecord,
            DbError::Infrastructure(msg) => GameError::InternalError(msg),
        }
    }
}
