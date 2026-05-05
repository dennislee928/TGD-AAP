# TGD-AAP (Taiwan Gov Data AI Automation Pipeline)

Rust-based pipeline for Taiwan government data ETL, gRPC inference serving, and alert-triggering client automation.

## Implemented Now

### Repository Scope
- Rust workspace with three binaries:
  - `data_engine` (ETL entrypoint)
  - `grpc_server` (tonic gRPC inference service)
  - `grpc_client` (scheduled inference caller + Telegram alert sender)
- Protobuf contract at `proto/inference.proto`.
- Toon serializer implementation in `src/data_engine/toon.rs`.
- Quantum bridge module scaffolding in `src/quantum_bridge/`.

### Current Runtime Behavior
- `data_engine`:
  - Pulls records from gov API URL (`GOV_API_URL` or default).
  - Cleans records (`value` must be numeric/non-NaN).
  - Serializes to Toon byte format.
  - Uploads bytes to Hugging Face dataset API.
- `grpc_server`:
  - Starts tonic server on `[::]:50051`.
  - Implements `Predict` and `PredictStream`.
  - `Predict` currently returns mock confidence (`0.42`).
- `grpc_client`:
  - Connects to gRPC server via `GRPC_SERVER_URL`.
  - Sends `PredictRequest`.
  - Triggers Telegram alert when threshold condition is met.

## Roadmap / Planned

- Production inference path (model loading + real feature inference).
- Stronger reliability controls (`error_for_status`, retry/backoff, timeout policy).
- Hardened auth/authz (move token out of payload metadata, interceptor/mTLS options).
- CI quality gates (fmt/clippy/test/security scanning).
- Expanded tests (unit/integration/smoke) and full operational docs.

## Quickstart

### Prerequisites
- Rust toolchain (edition 2021 project).
- Network access to:
  - Taiwan gov data endpoint.
  - Hugging Face dataset API.
  - Telegram bot API (for alerting path).

### Build

```bash
cargo build
```

### 1) Run ETL (`data_engine`)

```bash
export GOV_API_KEY="<gov-api-key-or-empty>"
export GOV_API_URL="https://data.gov.tw/api/v2/rest/datastore"  # optional, default exists
export HF_TOKEN="<huggingface-token>"
export HF_DATASET_REPO="tgd-aap/taiwan-gov-data"                # optional
cargo run --bin data_engine
```

Expected log output pattern:
- `ETL pipeline starting`
- `Fetched <N> records`
- `Cleaned data: <M> valid records`
- `Serialized to Toon format: <B> bytes`
- `Dataset uploaded to Hugging Face — ETL complete`

### 2) Run inference server (`grpc_server`)

```bash
cargo run --bin grpc_server
```

Expected log output pattern:
- `Starting gRPC server on [::]:50051`

### 3) Run inference client (`grpc_client`)

In a second shell (with server running):

```bash
export GRPC_SERVER_URL="http://[::1]:50051"
export GRPC_AUTH_TOKEN="<token-or-empty>"
export ALERT_THRESHOLD="0.8"   # optional, default 0.8

# Needed only when alert path is triggered
export TELEGRAM_BOT_TOKEN="<telegram-bot-token>"
export TELEGRAM_CHAT_ID="<telegram-chat-id>"

cargo run --bin grpc_client
```

Expected log output pattern:
- `Connecting to gRPC server at <url>`
- `Prediction received: confidence=0.4200, alert=false` (with current mock server logic)
- If threshold is met or server marks alert: `Alert sent via Telegram`

## Environment Variable Matrix

| Variable | Used by | Required | Default | Purpose |
|---|---|---:|---|---|
| `GOV_API_KEY` | `data_engine` | No | empty | API key header for gov API calls |
| `GOV_API_URL` | `data_engine` | No | `https://data.gov.tw/api/v2/rest/datastore` | Gov data endpoint |
| `HF_TOKEN` | `data_engine` | Yes | none | Bearer token for dataset upload |
| `HF_DATASET_REPO` | `data_engine` | No | `tgd-aap/taiwan-gov-data` | Hugging Face dataset repo |
| `GRPC_SERVER_URL` | `grpc_client` | Yes | none | gRPC server endpoint |
| `GRPC_AUTH_TOKEN` | `grpc_client` | No | empty | Token currently injected into request metadata map |
| `ALERT_THRESHOLD` | `grpc_client` | No | `0.8` | Confidence threshold for alerting |
| `TELEGRAM_BOT_TOKEN` | `grpc_client` | Conditional | none | Needed when sending alert |
| `TELEGRAM_CHAT_ID` | `grpc_client` | Conditional | none | Needed when sending alert |
| `RUST_LOG` | all bins | No | crate-level `tgd_aap=info` directive | Log filtering |

## Documentation Index
- Architecture: [docs/architecture.md](docs/architecture.md)
- ETL runbook: [docs/runbooks/etl.md](docs/runbooks/etl.md)
- Inference runbook: [docs/runbooks/inference.md](docs/runbooks/inference.md)
- Incident response runbook: [docs/runbooks/incident_response.md](docs/runbooks/incident_response.md)

## Notes on Current Limits
- Inference implementation is currently mock-based (`confidence=0.42`).
- Some external-call error paths are permissive and will be hardened in later tracks.
