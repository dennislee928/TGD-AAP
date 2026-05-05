//! data_engine/main.rs — ETL Pipeline binary entry point (Cron 1).
//!
//! Fetches data from Taiwan Government Open Data APIs, cleans and validates
//! records, serializes them to the Toon format, and uploads to Hugging Face.

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::time::{SystemTime, UNIX_EPOCH};

mod fetcher;
mod cleaner;
mod toon;

fn request_id(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{prefix}-{}-{ts}", std::process::id())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("tgd_aap=info".parse()?))
        .init();

    let request_id = request_id("etl");
    let dataset_version =
        std::env::var("DATASET_VERSION").unwrap_or_else(|_| "unknown".to_string());

    info!(
        request_id = %request_id,
        dataset_version = %dataset_version,
        "ETL pipeline starting"
    );

    let raw_data = fetcher::fetch_gov_data().await?;
    info!(
        request_id = %request_id,
        dataset_version = %dataset_version,
        record_count = raw_data.len(),
        "Fetched raw records"
    );

    let clean_data = cleaner::clean(raw_data)?;
    info!(
        request_id = %request_id,
        dataset_version = %dataset_version,
        valid_record_count = clean_data.len(),
        "Cleaned records"
    );

    let toon_bytes = toon::serialize(&clean_data)?;
    info!(
        request_id = %request_id,
        dataset_version = %dataset_version,
        toon_bytes = toon_bytes.len(),
        "Serialized to Toon"
    );

    toon::upload_to_huggingface(&toon_bytes).await?;
    info!(
        request_id = %request_id,
        dataset_version = %dataset_version,
        "Dataset uploaded to Hugging Face; ETL complete"
    );

    Ok(())
}
