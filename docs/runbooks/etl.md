# Runbook: ETL (`data_engine`)

## Purpose
Ingest Taiwan gov data, validate and clean it, serialize to Toon format, and upload to Hugging Face dataset storage.

## Binary
- `cargo run --bin data_engine`

## Required Inputs
- `HF_TOKEN` (required)

## Optional Inputs
- `GOV_API_KEY`
- `GOV_API_URL` (default: `https://data.gov.tw/api/v2/rest/datastore`)
- `HF_DATASET_REPO` (default: `tgd-aap/taiwan-gov-data`)
- `RUST_LOG`

## Standard Execution

```bash
export GOV_API_KEY="<optional>"
export GOV_API_URL="https://data.gov.tw/api/v2/rest/datastore"
export HF_TOKEN="<required>"
export HF_DATASET_REPO="tgd-aap/taiwan-gov-data"
cargo run --bin data_engine
```

## Healthy Signals
- `ETL pipeline starting`
- `Fetched <N> records`
- `Cleaned data: <M> valid records`
- `Serialized to Toon format: <B> bytes`
- `Dataset uploaded to Hugging Face — ETL complete`

## Failure Modes and Triage

1. `HF_TOKEN env var not set`
- Cause: missing required secret.
- Action: set `HF_TOKEN` and rerun.

2. Network/HTTP failure while fetching gov data
- Cause: endpoint unreachable, TLS/network issue.
- Action: confirm `GOV_API_URL`, connectivity, and API availability.

3. JSON parse fallback returns zero records
- Symptom: unexpectedly low `Fetched` count.
- Action: inspect upstream response schema; current code can default to empty parse.

4. Upload failure (`Failed to upload to Hugging Face`)
- Cause: invalid token, repo permissions, API response failure.
- Action: validate `HF_TOKEN` scope and `HF_DATASET_REPO` correctness.

## Post-Run Checks
- Confirm run completion log.
- Verify dataset update in target HF dataset repository.

## Escalation
Escalate when:
- Repeated upload failures with valid credentials.
- Persistent zero/near-zero record fetch after confirming endpoint health.
