//! data_engine/toon.rs — Toon format serialization and Hugging Face upload.
//!
//! The Toon format is a lightweight binary-wrapped JSON-lines format used
//! for efficient dataset storage and cross-node transfer.

use anyhow::{anyhow, Context, Result};
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

/// Deserialize Toon bytes into clean records.
///
/// Validates magic/version/count and JSON-lines payload shape.
pub fn deserialize(data: &[u8]) -> Result<Vec<CleanRecord>> {
    const HEADER_LEN: usize = 9;

    if data.len() < HEADER_LEN {
        return Err(anyhow!("toon payload too short: {}", data.len()));
    }

    if &data[..4] != TOON_MAGIC {
        return Err(anyhow!("invalid toon magic"));
    }

    let version = data[4];
    if version != TOON_VERSION {
        return Err(anyhow!("unsupported toon version: {}", version));
    }

    let mut count_bytes = [0_u8; 4];
    count_bytes.copy_from_slice(&data[5..9]);
    let expected_count = u32::from_le_bytes(count_bytes) as usize;

    let payload = std::str::from_utf8(&data[9..]).context("toon payload is not valid UTF-8")?;
    let mut out = Vec::with_capacity(expected_count);

    for line in payload.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let value: serde_json::Value =
            serde_json::from_str(line).context("invalid toon JSON line")?;

        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("toon record missing string id"))?
            .to_string();
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("toon record missing string name"))?
            .to_string();
        let value = value
            .get("value")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| anyhow!("toon record missing numeric value"))?;

        out.push(CleanRecord { id, name, value });
    }

    if out.len() != expected_count {
        return Err(anyhow!(
            "toon record count mismatch: expected {}, got {}",
            expected_count,
            out.len()
        ));
    }

    Ok(out)
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

#[cfg(test)]
mod tests {
    use super::{deserialize, serialize};
    use crate::data_engine::cleaner::CleanRecord;

    #[test]
    fn serialize_deserialize_round_trip() {
        let records = vec![
            CleanRecord {
                id: "A-1".to_string(),
                name: "Alpha".to_string(),
                value: 12.5,
            },
            CleanRecord {
                id: "B-2".to_string(),
                name: "Beta".to_string(),
                value: 0.25,
            },
        ];

        let bytes = serialize(&records).expect("serialize should succeed");
        let decoded = deserialize(&bytes).expect("deserialize should succeed");

        assert_eq!(decoded.len(), records.len());
        for (lhs, rhs) in decoded.iter().zip(records.iter()) {
            assert_eq!(lhs.id, rhs.id);
            assert_eq!(lhs.name, rhs.name);
            assert!((lhs.value - rhs.value).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn deserialize_rejects_invalid_magic() {
        let mut bytes = b"BADO".to_vec();
        bytes.extend_from_slice(&[1, 0, 0, 0, 0]);
        let err = deserialize(&bytes).expect_err("invalid magic should fail");
        assert!(err.to_string().contains("invalid toon magic"));
    }

    #[test]
    fn deserialize_rejects_count_mismatch() {
        let bad = b"TOON\x01\x02\x00\x00\x00{\"id\":\"x\",\"name\":\"n\",\"value\":1}\n";
        let err = deserialize(bad).expect_err("count mismatch should fail");
        assert!(err.to_string().contains("count mismatch"));
    }
}
