//! data_engine/cleaner.rs — Data cleaning and validation routines.

use anyhow::Result;
use tracing::warn;

use super::fetcher::RawRecord;

/// A validated, clean record ready for serialization.
#[derive(Debug, serde::Serialize)]
pub struct CleanRecord {
    pub id: String,
    pub name: String,
    pub value: f64,
}

/// Validate and clean raw records, discarding malformed entries.
pub fn clean(raw: Vec<RawRecord>) -> Result<Vec<CleanRecord>> {
    let mut clean = Vec::with_capacity(raw.len());

    for record in raw {
        match record.value.as_f64() {
            Some(v) if !v.is_nan() => {
                clean.push(CleanRecord {
                    id: record.id,
                    name: record.name,
                    value: v,
                });
            }
            _ => {
                warn!("Discarding malformed record id={}", record.id);
            }
        }
    }

    Ok(clean)
}
