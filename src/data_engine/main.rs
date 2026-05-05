//! data_engine/main.rs — ETL Pipeline binary entry point (Cron 1).
//!
//! Fetches data from Taiwan Government Open Data APIs, cleans and validates
//! records, serializes them to the Toon format, and uploads to Hugging Face.

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod fetcher;
mod cleaner;
mod toon;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("tgd_aap=info".parse()?))
        .init();

    info!("ETL pipeline starting");

    let raw_data = fetcher::fetch_gov_data().await?;
    info!("Fetched {} records", raw_data.len());

    let clean_data = cleaner::clean(raw_data)?;
    info!("Cleaned data: {} valid records", clean_data.len());

    let toon_bytes = toon::serialize(&clean_data)?;
    info!("Serialized to Toon format: {} bytes", toon_bytes.len());

    toon::upload_to_huggingface(&toon_bytes).await?;
    info!("Dataset uploaded to Hugging Face — ETL complete");

    Ok(())
}
