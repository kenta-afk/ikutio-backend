use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use futures::{SinkExt, StreamExt};

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::GameService;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::{StartGameReply, StartGameRequest};

#[utoipa::path(
    get,
    path = "/start_game",
    responses(
        (status = 200, description = "Start game successfully", body = StartGameReply),
    ),
)]

pub async fn start_game<GSC>(
    ws: WebSocketUpgrade,
    authenticated_user: AuthenticatedUser,
    State(GameService(mut gsc)): State<GameService<GSC>>,
) -> AppResult<Response>
where
    GSC: GameServiceClientTrait, {
    let request = StartGameRequest {};

    let response = gsc
        .start_game(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;

    Ok(ws.on_upgrade(move |socket| async move {
        let (mut sender, _receiver) = socket.split();

        let mut stream = response;
        while let Some(item) = StreamExt::next(&mut stream).await {
            match item {
                Ok(reply) => {
                    let message = serde_json::to_string(&reply).unwrap();
                    if sender.send(axum::extract::ws::Message::Text(message.into())).await.is_err()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        let _ = sender.close().await;
    }))
}
