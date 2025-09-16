mod internal;
mod proto;

use std::env;

use aws_sdk_dynamodb::Client;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tracing::Level;

use crate::internal::application::use_case::GameServiceImpl;
use crate::internal::domain::game_repository::GameRepository;
use crate::internal::infrastructure::game_repository::GameRepositoryImpl;
use crate::internal::infrastructure::time_generator::TimeGeneratorImpl;
use crate::internal::infrastructure::uuid_generator::UuidGeneratorImpl;
use crate::proto::game_service_server::GameServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("dev/.env").expect("Failed to load .env file");

    let log_level = match env::var("LOG_LEVEL").as_deref() {
        Ok("INFO") => Level::INFO,
        Ok("DEBUG") => Level::DEBUG,
        Ok("ERROR") => Level::ERROR,
        _ => Level::DEBUG,
    };

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(log_level)
        .with_thread_ids(true)
        .with_thread_names(true)
        .json()
        .init();

    // ローカルのDynamoDBに接続するための設定を構築
    let shared_config = aws_config::from_env().load().await;
    let config_builder = aws_sdk_dynamodb::config::Builder::from(&shared_config);
    let endpoint_url = env::var("ENDPOINT_URL").expect("ENDPOINT_URL must be set");

    // エンドポイントURLをローカルのDynamoDBに指定
    let config = config_builder.endpoint_url(endpoint_url).build();
    //　設定を反映してクライアントを再構築
    let client = Client::from_conf(config);

    let game_service = env::var("GAMESERVICE").expect("AUTHSERVICE must be set");
    tracing::info!("Starting AuthService at {}", game_service);

    let game_service =
        GameServiceImpl::new(GameRepositoryImpl::new(client), UuidGeneratorImpl, TimeGeneratorImpl);

    tracing::info!("GameService is running...");

    let listener = TcpListener::bind("0.0.0.0:50054").await.expect("Failed to bind to address");

    let handler = tokio::spawn(
        Server::builder()
            .add_service(GameServiceServer::new(game_service))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );

    tracing::info!("GameService has started successfully.");

    let _ = handler.await?;

    Ok(())
}
