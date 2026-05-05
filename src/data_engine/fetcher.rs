//! data_engine/fetcher.rs — Taiwan Government Open Data API client.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
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
    const REQUEST_TIMEOUT_SECS: u64 = 15;
    const MAX_RETRIES: usize = 3;
    const INITIAL_BACKOFF_MS: u64 = 500;

    let api_key = std::env::var("GOV_API_KEY").unwrap_or_default();
    let base_url = std::env::var("GOV_API_URL")
        .unwrap_or_else(|_| "https://data.gov.tw/api/v2/rest/datastore".to_string());

    info!("Fetching data from {}", base_url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        .context("Failed to build HTTP client")?;

    let mut last_error: Option<anyhow::Error> = None;

    for attempt in 1..=MAX_RETRIES {
        let response = client
            .get(&base_url)
            .header("X-API-Key", &api_key)
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    let records: Vec<RawRecord> = resp
                        .json()
                        .await
                        .with_context(|| {
                            format!(
                                "Failed to parse government API response JSON (url={}, attempt={})",
                                base_url, attempt
                            )
                        })?;
                    return Ok(records);
                }

                if is_transient_status(status) && attempt < MAX_RETRIES {
                    let backoff_ms = INITIAL_BACKOFF_MS * (1_u64 << (attempt - 1));
                    info!(
                        "Transient HTTP status {} from {} (attempt {}/{}), retrying in {}ms",
                        status, base_url, attempt, MAX_RETRIES, backoff_ms
                    );
                    sleep(Duration::from_millis(backoff_ms)).await;
                    continue;
                }

                return Err(resp
                    .error_for_status()
                    .err()
                    .map(anyhow::Error::from)
                    .unwrap_or_else(|| {
                        anyhow!(
                            "Government API request failed with non-success status {} (url={}, attempt={}/{})",
                            status,
                            base_url,
                            attempt,
                            MAX_RETRIES
                        )
                    }))
                .with_context(|| {
                    format!(
                        "Non-retryable HTTP status from government API (url={}, attempt={}/{})",
                        base_url, attempt, MAX_RETRIES
                    )
                });
            }
            Err(err) => {
                let transient = err.is_timeout() || err.is_connect() || err.is_request();
                if transient && attempt < MAX_RETRIES {
                    let backoff_ms = INITIAL_BACKOFF_MS * (1_u64 << (attempt - 1));
                    info!(
                        "Transient request error from {} (attempt {}/{}): {}. Retrying in {}ms",
                        base_url, attempt, MAX_RETRIES, err, backoff_ms
                    );
                    last_error = Some(err.into());
                    sleep(Duration::from_millis(backoff_ms)).await;
                    continue;
                }

                return Err(anyhow!(err)).with_context(|| {
                    format!(
                        "Government API request failed (url={}, attempt={}/{})",
                        base_url, attempt, MAX_RETRIES
                    )
                });
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow!("Government API request failed after retries")))
}

fn is_transient_status(status: reqwest::StatusCode) -> bool {
    status.is_server_error()
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || status == reqwest::StatusCode::REQUEST_TIMEOUT
}
