use std::pin::Pin;

use tokio_stream::Stream;
use tonic::{Request, Response, Status};

use crate::internal::domain::game_repository::GameRepository;
use crate::internal::domain::models::error::GameError;
use crate::internal::domain::models::game::Game;
use crate::internal::domain::models::id::UserId;
use crate::internal::domain::service::time::TimeDomainService;
use crate::internal::infrastructure::time_generator::TimeGenerator;
use crate::internal::infrastructure::uuid_generator::UuidGenerator;
use crate::proto::game_service_server::GameService;
use crate::proto::{StartGameReply, StartGameRequest};

pub struct GameServiceImpl<GR, UG, TG>
where
    GR: GameRepository,
    UG: UuidGenerator,
    TG: TimeGenerator, {
    game_repository: GR,
    uuid_generator: UG,
    time_generator: TG,
}

impl<GR, UG, TG> GameServiceImpl<GR, UG, TG>
where
    GR: GameRepository,
    UG: UuidGenerator,
    TG: TimeGenerator,
{
    pub fn new(game_repository: GR, uuid_generator: UG, time_generator: TG) -> Self {
        Self { game_repository, uuid_generator, time_generator }
    }

    pub async fn start_game(
        &self,
        request: Request<StartGameRequest>,
    ) -> Result<Pin<Box<dyn Stream<Item = u32> + Send + 'static>>, GameError> {
        // gRPCのmetadataからuser_idを取得
        let metadata = request.metadata();
        let user_id_str =
            metadata.get("user_id").and_then(|v| v.to_str().ok()).ok_or(GameError::InvalidInput)?;

        let user_uuid = uuid::Uuid::parse_str(user_id_str).map_err(|_| GameError::InvalidInput)?;
        let user_id = UserId::from_uuid(user_uuid);

        let game = Game::new(user_id, &self.uuid_generator);
        self.game_repository.save(game).await?;

        let start_time = self.time_generator.now();
        let game_duration = self.time_generator.create_duration(160);

        let time_stream =
            TimeDomainService::get_time_stream(start_time, game_duration, &self.time_generator);
        Ok(Box::pin(time_stream))
    }
}

#[tonic::async_trait]
impl<GR, UG, TG> GameService for GameServiceImpl<GR, UG, TG>
where
    GR: GameRepository + Send + Sync,
    UG: UuidGenerator + Send + Sync,
    TG: TimeGenerator + Send + Sync,
{
    type StartGameStream =
        Pin<Box<dyn Stream<Item = Result<StartGameReply, Status>> + Send + 'static>>;

    async fn start_game(
        &self,
        request: Request<StartGameRequest>,
    ) -> Result<Response<Self::StartGameStream>, Status> {
        match GameServiceImpl::start_game(self, request).await {
            Ok(time_stream) => {
                let mapped_stream = tokio_stream::StreamExt::map(time_stream, |timer_value| {
                    tracing::info!("Sending timer value: {}", timer_value);
                    Ok(StartGameReply { timer: timer_value })
                });

                let mut response = Response::new(Box::pin(mapped_stream)
                    as Pin<Box<dyn Stream<Item = Result<StartGameReply, Status>> + Send>>);

                // Disable all forms of buffering to ensure real-time streaming
                response.metadata_mut().insert("x-accel-buffering", "no".parse().unwrap());
                response.metadata_mut().insert(
                    "cache-control",
                    "no-cache, no-store, must-revalidate".parse().unwrap(),
                );
                response.metadata_mut().insert("pragma", "no-cache".parse().unwrap());
                response.metadata_mut().insert("expires", "0".parse().unwrap());
                response
                    .metadata_mut()
                    .insert("x-content-type-options", "nosniff".parse().unwrap());
                Ok(response)
            }
            Err(game_error) => Err(game_error.into()),
        }
    }
}
