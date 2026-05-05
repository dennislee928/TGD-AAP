# TGD-AAP (Taiwan Gov Data AI Automation Pipeline)

Rust-based pipeline for Taiwan government data ETL, gRPC inference serving, and alert-triggering client automation.

## Implemented Now

### Repository Scope
- Rust workspace with three binaries:
  - `data_engine` (ETL entrypoint)
  - `grpc_server` (tonic gRPC inference service)
  - `grpc_client` (scheduled inference caller + Telegram alert sender)
- Protobuf contract at `proto/inference.proto`.
- Toon serialize/deserialize implementation in `src/data_engine/toon.rs`.
- Quantum bridge scoring path with deterministic fallback in `src/quantum_bridge/`.
- Baseline CI workflow and test fixtures.

### Current Runtime Behavior
- `data_engine`:
  - Fetches gov records with timeout/retry/backoff and strict HTTP/JSON error handling.
  - Cleans records with deterministic schema rejection.
  - Serializes to Toon and uploads to Hugging Face dataset API.
- `grpc_server`:
  - Starts tonic server on `[::]:50051`.
  - Implements `Predict` and `PredictStream`.
  - Uses quantum-first feature scoring with classical fallback when quantum runtime is unavailable.
  - Supports token validation via `GRPC_EXPECTED_TOKEN`.
- `grpc_client`:
  - Connects via `GRPC_SERVER_URL`.
  - Sends bearer token in gRPC metadata (`authorization`).
  - Triggers Telegram alert when threshold condition is met.

## Roadmap / Planned
- Optional mTLS/interceptor-based auth hardening.
- End-to-end deployment automation and smoke tests in hosted runtime.
- Additional observability (metrics and tracing export).
- Expanded integration/load testing.

## Quickstart

### Prerequisites
- Rust toolchain (edition 2021 project).
- `protoc` available for protobuf codegen at build time.
- Runtime dependencies as needed:
  - Taiwan gov data endpoint
  - Hugging Face dataset API
  - Telegram bot API
  - `holyqasm` binary for quantum execution path (fallback exists if unavailable)

### Build

```bash
cargo build
```

### 1) Run ETL (`data_engine`)

```bash
export GOV_API_KEY="<gov-api-key-or-empty>"
export GOV_API_URL="https://data.gov.tw/api/v2/rest/datastore"
export HF_TOKEN="<huggingface-token>"
export HF_DATASET_REPO="tgd-aap/taiwan-gov-data"
export DATASET_VERSION="dev"
cargo run --bin data_engine
```

### 2) Run inference server (`grpc_server`)

```bash
export MODEL_VERSION="dev"
export DATASET_VERSION="dev"
export GRPC_EXPECTED_TOKEN="<shared-token-or-empty>"
cargo run --bin grpc_server
```

### 3) Run inference client (`grpc_client`)

```bash
export GRPC_SERVER_URL="http://[::1]:50051"
export GRPC_AUTH_TOKEN="<token-or-empty>"
export ALERT_THRESHOLD="0.8"
export TELEGRAM_BOT_TOKEN="<telegram-bot-token>"
export TELEGRAM_CHAT_ID="<telegram-chat-id>"
cargo run --bin grpc_client
```

## Environment Variable Matrix

| Variable | Used by | Required | Default | Purpose |
|---|---|---:|---|---|
| `GOV_API_KEY` | `data_engine` | No | empty | API key header for gov API calls |
| `GOV_API_URL` | `data_engine` | No | `https://data.gov.tw/api/v2/rest/datastore` | Gov data endpoint |
| `HF_TOKEN` | `data_engine` | Yes | none | Bearer token for dataset upload |
| `HF_DATASET_REPO` | `data_engine` | No | `tgd-aap/taiwan-gov-data` | Hugging Face dataset repo |
| `DATASET_VERSION` | `data_engine`, `grpc_server` | No | `unknown` | Dataset version log context |
| `MODEL_VERSION` | `grpc_server` | No | `unknown` | Model version log context |
| `GRPC_EXPECTED_TOKEN` | `grpc_server` | No | empty | Expected bearer token for auth validation |
| `GRPC_SERVER_URL` | `grpc_client` | Yes | none | gRPC server endpoint |
| `GRPC_AUTH_TOKEN` | `grpc_client` | No | empty | Bearer token sent as gRPC metadata |
| `ALERT_THRESHOLD` | `grpc_client` | No | `0.8` | Confidence threshold for alerting |
| `TELEGRAM_BOT_TOKEN` | `grpc_client` | Conditional | none | Needed when sending alert |
| `TELEGRAM_CHAT_ID` | `grpc_client` | Conditional | none | Needed when sending alert |

## Documentation Index
- Architecture: [docs/architecture.md](docs/architecture.md)
- Toon spec: [docs/toon-spec.md](docs/toon-spec.md)
- ETL runbook: [docs/runbooks/etl.md](docs/runbooks/etl.md)
- Inference runbook: [docs/runbooks/inference.md](docs/runbooks/inference.md)
- Incident response runbook: [docs/runbooks/incident_response.md](docs/runbooks/incident_response.md)
- Progress tracker: [docs/dev_plans/implementation_1_progress.md](docs/dev_plans/implementation_1_progress.md)
