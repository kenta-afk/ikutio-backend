use axum::body::Body;
use axum::extract::State;
use axum::http::Response;
use futures::StreamExt;
use tokio_stream::StreamExt as TokioStreamExt;

use crate::routes::extractor::AuthenticatedUser;
use crate::routes::response::{AppError, AppResult};
use crate::routes::state::GameService;
use crate::services::StartGameRequest;
use crate::services::gameserviceclient::GameServiceClientTrait;

pub async fn start_game<GSC>(
    authenticated_user: AuthenticatedUser,
    State(GameService(mut gsc)): State<GameService<GSC>>,
) -> AppResult<Response<Body>>
where
    GSC: GameServiceClientTrait, {
    let request = StartGameRequest {};

    let response = gsc
        .start_game(authenticated_user.user_id, request)
        .await
        .map_err(AppError::internal_error)?;

    // StreamingをHTTPレスポンスに直接変換
    let body = Body::from_stream(StreamExt::map(response, |item| {
        item.map(|reply| {
            let mut data = serde_json::to_vec(&reply).unwrap();
            data.push(b'\n');
            bytes::Bytes::from(data)
        })
        .map_err(std::io::Error::other)
    }));

    Ok(Response::builder()
        .header("content-type", "application/x-ndjson")
        .header("cache-control", "no-cache")
        .header("connection", "keep-alive")
        .header("x-accel-buffering", "no")
        .body(body)
        .unwrap())
}
