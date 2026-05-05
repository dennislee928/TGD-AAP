//! data_engine/fetcher.rs — Taiwan Government Open Data API client.

use anyhow::Result;
use serde::Deserialize;
use tracing::info;

/// A single raw record returned by the government API.
#[derive(Debug, Deserialize)]
pub struct RawRecord {
    pub id: String,
    pub name: String,
    pub value: serde_json::Value,
}

/// Fetch records from the Taiwan Government Open Data portal.
pub async fn fetch_gov_data() -> Result<Vec<RawRecord>> {
    let api_key = std::env::var("GOV_API_KEY").unwrap_or_default();
    let base_url = std::env::var("GOV_API_URL")
        .unwrap_or_else(|_| "https://data.gov.tw/api/v2/rest/datastore".to_string());

    info!("Fetching data from {}", base_url);

    let client = reqwest::Client::new();
    let records: Vec<RawRecord> = client
        .get(&base_url)
        .header("X-API-Key", api_key)
        .send()
        .await?
        .json()
        .await
        .unwrap_or_default();

    Ok(records)
}
