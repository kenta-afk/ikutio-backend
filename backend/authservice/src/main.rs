mod application;
mod domain;
mod infrastructure;
mod proto;

use std::env;

use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tracing::Level;

use crate::application::use_case::AuthServiceImpl;
use crate::domain::auth_repository::AuthRepository;
use crate::infrastructure::auth_repository::AuthRepositoryImpl;
use crate::infrastructure::jwt_generator::{JwtGenerator, JwtGeneratorImpl};
use crate::infrastructure::uuid_generator::UuidGeneratorImpl;
use crate::proto::auth_service_server::AuthServiceServer;

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

    let auth_service = env::var("AUTHSERVICE").expect("AUTHSERVICE must be set");
    tracing::info!("Starting AuthService at {}", auth_service);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&db_url).await.expect("Failed to create pool.");

    let auth_service = AuthServiceImpl::new(
        AuthRepositoryImpl::new(pool),
        UuidGeneratorImpl,
        JwtGeneratorImpl::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_ref()),
    );

    tracing::info!("AuthService is running...");

    let listener = TcpListener::bind("0.0.0.0:50053").await.expect("Failed to bind to address");

    let handler = tokio::spawn(
        Server::builder()
            .add_service(AuthServiceServer::new(auth_service))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );

    tracing::info!("AuthService has started successfully.");

    let _ = handler.await?;

    Ok(())
}
