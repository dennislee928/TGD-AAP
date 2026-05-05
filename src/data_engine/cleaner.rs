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

fn sanitize_required_text(input: String) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_finite_value(value: serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64().filter(|v| v.is_finite()),
        serde_json::Value::String(s) => s
            .trim()
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite()),
        _ => None,
    }
}

/// Validate and clean raw records, discarding malformed entries.
pub fn clean(raw: Vec<RawRecord>) -> Result<Vec<CleanRecord>> {
    let mut clean = Vec::with_capacity(raw.len());

    for record in raw {
        let RawRecord { id, name, value } = record;
        let id = match sanitize_required_text(id) {
            Some(v) => v,
            None => {
                warn!("Discarding malformed record: reason=empty_id");
                continue;
            }
        };

        let name = match sanitize_required_text(name) {
            Some(v) => v,
            None => {
                warn!("Discarding malformed record id={}: reason=empty_name", id);
                continue;
            }
        };

        let value = match parse_finite_value(value) {
            Some(v) => v,
            None => {
                warn!(
                    "Discarding malformed record id={}: reason=non_finite_or_non_numeric_value",
                    id
                );
                continue;
            }
        };

        clean.push(CleanRecord {
            id,
            name,
            value,
        });
    }

    Ok(clean)
}
