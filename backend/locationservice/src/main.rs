mod internal;
mod proto;

use std::env;

use aws_sdk_dynamodb::Client;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tracing::Level;

use crate::internal::application::use_case::LocationServiceImpl;
use crate::internal::domain::location_repository::LocationRepository;
use crate::internal::infrastructure::location_repository::LocationRepositoryImpl;
use crate::internal::infrastructure::uuid_generator::UuidGeneratorImpl;
use crate::proto::location_service_server::LocationServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("dev/.env").ok();

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
    let endpoint_url = env::var("ENDPOINT_URL").ok();

    // エンドポイントURLをローカルのDynamoDBに指定
    let config = if let Some(url) = endpoint_url {
        config_builder.endpoint_url(url).build()
    } else {
        config_builder.build()
    };
    //　設定を反映してクライアントを再構築
    let client = Client::from_conf(config);

    let location_service = env::var("LOCATIONSERVICE").expect("LOCATIONSERVICE must be set");
    tracing::info!("Starting LlocationService at {}", location_service);

    let game_service =
        LocationServiceImpl::new(LocationRepositoryImpl::new(client), UuidGeneratorImpl);

    tracing::info!("LocationService is running...");

    let listener = TcpListener::bind("0.0.0.0:50055").await.expect("Failed to bind to address");

    let handler = tokio::spawn(
        Server::builder()
            .add_service(LocationServiceServer::new(game_service))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );

    tracing::info!("LocationService has started successfully.");

    let _ = handler.await?;

    Ok(())
}
