//! data_engine/toon.rs — Toon format serialization and Hugging Face upload.
//!
//! The Toon format is a lightweight binary-wrapped JSON-lines format used
//! for efficient dataset storage and cross-node transfer.

use anyhow::{Context, Result};
use tracing::info;

use super::cleaner::CleanRecord;

/// Magic bytes that identify a Toon-formatted file.
const TOON_MAGIC: &[u8] = b"TOON";
const TOON_VERSION: u8 = 1;

/// Serialize clean records into the Toon binary format.
///
/// Layout: [magic 4B][version 1B][record_count 4B LE][newline-delimited JSON payload]
pub fn serialize(records: &[CleanRecord]) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();

    buf.extend_from_slice(TOON_MAGIC);
    buf.push(TOON_VERSION);

    let count = records.len() as u32;
    buf.extend_from_slice(&count.to_le_bytes());

    for record in records {
        let line = serde_json::to_string(record)? + "\n";
        buf.extend_from_slice(line.as_bytes());
    }

    Ok(buf)
}

/// Upload a Toon-formatted byte buffer to Hugging Face Datasets.
pub async fn upload_to_huggingface(data: &[u8]) -> Result<()> {
    let hf_token = std::env::var("HF_TOKEN").context("HF_TOKEN env var not set")?;
    let repo = std::env::var("HF_DATASET_REPO")
        .unwrap_or_else(|_| "tgd-aap/taiwan-gov-data".to_string());

    let url = format!(
        "https://huggingface.co/api/datasets/{}/upload",
        repo
    );

    info!("Uploading {} bytes to {}", data.len(), url);

    reqwest::Client::new()
        .post(&url)
        .bearer_auth(hf_token)
        .body(data.to_vec())
        .send()
        .await
        .context("Failed to upload to Hugging Face")?;

    Ok(())
}
