//! main.rs — gRPC Server entry point for TGD-AAP.
//!
//! Starts the tonic-based gRPC server that serves inference requests
//! from GitHub Actions cron jobs and other clients.

use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub mod generated {
    tonic::include_proto!("inference");
}

mod data_engine;
mod quantum_bridge;

use generated::inference_service_server::InferenceServiceServer;

mod inference_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("tgd_aap=info".parse()?))
        .init();

    let addr = "[::]:50051".parse()?;
    info!("Starting gRPC server on {}", addr);

    let handler = inference_handler::InferenceHandler::new();

    Server::builder()
        .add_service(InferenceServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
