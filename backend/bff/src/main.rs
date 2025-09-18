mod routes;
mod services;

use std::env;

use services::auth_service_client::AuthServiceClient;
use services::profileserviceclient::ProfileServiceClient;
use tokio::net::TcpListener;
use tracing::Level;

use crate::services::game_service_client::GameServiceClient;
use crate::services::location_service_client::LocationServiceClient;

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

    let host = env::var("BFF").expect("BFF must be set");
    let auth_service = env::var("AUTHSERVICE").expect("AUTHSERVICE must be set");
    let profile_service = env::var("PROFILESERVICE").expect("PROFILESERVICE must be set");
    let game_service = env::var("GAMESERVICE").expect("GAMESERVICE must be set");
    let location_service = env::var("LOCATIONSERVICE").expect("LOCATIONSERVICE must be set");

    let auth_client = loop {
        match AuthServiceClient::connect(format!("http://{auth_service}")).await {
            Ok(client) => {
                tracing::info!("Connected to AuthService at {}", auth_service);
                break client;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to connect to AuthService: {}. Retrying in 5 seconds...",
                    e
                );

                tracing::info!("Retrying connection to AuthService...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let profile_client = loop {
        match ProfileServiceClient::connect(format!("http://{profile_service}")).await {
            Ok(client) => {
                tracing::info!("Connected to ProfileService at {}", profile_service);
                break client;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to connect to ProfileService: {}. Retrying in 5 seconds...",
                    e
                );

                tracing::info!("Retrying connection to ProfileService...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let game_client = loop {
        match GameServiceClient::connect(format!("http://{game_service}")).await {
            Ok(client) => {
                tracing::info!("Connected to GameService at {}", game_service);
                break client;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to connect to GameService: {}. Retrying in 5 seconds...",
                    e
                );

                tracing::info!("Retrying connection to GameService...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let location_client = loop {
        match LocationServiceClient::connect(format!("http://{location_service}")).await {
            Ok(client) => {
                tracing::info!("Connected to LocationService at {}", location_service);
                break client;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to connect to LocationService: {}. Retrying in 5 seconds...",
                    e
                );

                tracing::info!("Retrying connection to LocationService...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let router =
        routes::router::create_routes(auth_client, profile_client, game_client, location_client);

    let listener = TcpListener::bind(host).await?;

    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}
