mod services;
mod routes;

use services::auth_service_client::AuthServiceClient;
use std::env;
use tracing::Level;
use tokio::net::TcpListener;

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

    let host = env::var("BFF").expect("BFF must be set");
    let auth_service = env::var("AUTHSERVICE").expect("AUTHSERVICE must be set");

    let authservice_client = loop {
        match AuthServiceClient::connect(format!("http://{auth_service}")).await {
            Ok(client) => {
                tracing::info!("Connected to AuthService at {}", auth_service);
                break client;
            }
            Err(e) => {
                tracing::error!("Failed to connect to AuthService: {}. Retrying in 5 seconds...", e);

                tracing::info!("Retrying connection to AuthService...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let router = routes::router::create_routes(authservice_client);

    let listener = TcpListener::bind(host).await?;

    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())


}
