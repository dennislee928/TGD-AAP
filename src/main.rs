//! main.rs — gRPC Server entry point for TGD-AAP.
//!
//! Starts the tonic-based gRPC server that serves inference requests
//! from GitHub Actions cron jobs and other clients.

use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod generated {
    tonic::include_proto!("inference");
}

mod data_engine;
mod quantum_bridge;

use generated::inference_service_server::InferenceServiceServer;

mod inference_handler;

fn request_id(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{prefix}-{}-{ts}", std::process::id())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("tgd_aap=info".parse()?))
        .init();

    let request_id = request_id("grpc-server");
    let model_version = std::env::var("MODEL_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let dataset_version =
        std::env::var("DATASET_VERSION").unwrap_or_else(|_| "unknown".to_string());

    let addr = "[::]:50051".parse()?;
    info!(
        request_id = %request_id,
        model_version = %model_version,
        dataset_version = %dataset_version,
        server_addr = %addr,
        "Starting gRPC server"
    );

    let handler = inference_handler::InferenceHandler::new();

    Server::builder()
        .add_service(InferenceServiceServer::new(handler))
        .serve(addr)
        .await?;

    info!(
        request_id = %request_id,
        model_version = %model_version,
        dataset_version = %dataset_version,
        "gRPC server stopped"
    );

    Ok(())
}
