use std::env;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::Response;
use futures::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

use crate::routes::extractor::Claims;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::GameService;
use crate::services::gameserviceclient::GameServiceClientTrait;
use crate::services::StartGameRequest;

#[derive(Debug, Deserialize)]
pub struct JwtQuery {
    pub jwt: String,
}

fn extract_user_id_from_token(token: &str) -> Option<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &decoding_key, &validation).ok()?;
    Some(token_data.claims.sub)
}

pub async fn start_game<GSC>(
    ws: WebSocketUpgrade,
    Query(params): Query<JwtQuery>,
    State(GameService(gsc)): State<GameService<GSC>>,
) -> AppResult<Response>
where
    GSC: GameServiceClientTrait, {
    let user_id = extract_user_id_from_token(&params.jwt)
        .ok_or_else(|| AppError::unauthorized("Invalid JWT token"))?;

    Ok(ws.on_upgrade(move |socket| async move {
        handle_websocket(socket, gsc, user_id).await;
    }))
}

async fn handle_websocket<GSC>(socket: WebSocket, mut gsc: GSC, user_id: String)
where
    GSC: GameServiceClientTrait, {
    let (mut sender, _receiver) = socket.split();

    let request = StartGameRequest {};

    match gsc.start_game(user_id, request).await {
        Ok(mut stream) => {
            while let Some(item) = StreamExt::next(&mut stream).await {
                match item {
                    Ok(reply) => {
                        let message = serde_json::to_string(&reply).unwrap();
                        if sender.send(Message::Text(message.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
        Err(_) => {
            let _ = sender.send(Message::Text("{\"error\":\"Failed to start game\"}".into())).await;
        }
    }

    let _ = sender.close().await;
}
