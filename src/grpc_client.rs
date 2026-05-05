//! grpc_client.rs — gRPC Client binary for TGD-AAP.
//!
//! Invoked by GitHub Actions Cron 2. Sends an inference request to the
//! hosted gRPC server and triggers an alert if the threshold is exceeded.

use anyhow::{Context, Result};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod generated {
    tonic::include_proto!("inference");
}

use generated::{inference_service_client::InferenceServiceClient, PredictRequest};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("tgd_aap=info".parse()?))
        .init();

    let server_url =
        std::env::var("GRPC_SERVER_URL").context("GRPC_SERVER_URL env var not set")?;
    let auth_token = std::env::var("GRPC_AUTH_TOKEN").unwrap_or_default();
    let threshold: f32 = std::env::var("ALERT_THRESHOLD")
        .unwrap_or_else(|_| "0.8".to_string())
        .parse()
        .unwrap_or(0.8);

    info!("Connecting to gRPC server at {}", server_url);
    let mut client = InferenceServiceClient::connect(server_url).await?;

    let request = tonic::Request::new(PredictRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        features: vec![],
        metadata: [("auth_token".to_string(), auth_token)].into(),
    });

    let response = client.predict(request).await?.into_inner();
    info!(
        "Prediction received: confidence={:.4}, alert={}",
        response.confidence, response.alert_triggered
    );

    if response.alert_triggered || response.confidence >= threshold {
        warn!("Alert threshold reached — triggering notification");
        send_alert(&response.message).await?;
    }

    Ok(())
}

async fn send_alert(message: &str) -> Result<()> {
    let bot_token =
        std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN env var not set")?;
    let chat_id =
        std::env::var("TELEGRAM_CHAT_ID").context("TELEGRAM_CHAT_ID env var not set")?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let client = reqwest::Client::new();
    client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": format!("[TGD-AAP Alert] {}", message),
        }))
        .send()
        .await?;

    info!("Alert sent via Telegram");
    Ok(())
}
