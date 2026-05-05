# Architecture

## Overview
TGD-AAP is a Rust-first pipeline composed of three executable paths:
1. ETL ingestion and dataset publication (`data_engine`).
2. gRPC inference serving (`grpc_server`).
3. Scheduled or ad-hoc inference invocation and alerting (`grpc_client`).

## Component Diagram

```text
+-------------------+      HTTPS       +------------------------------+
| data_engine       | ---------------> | Taiwan Gov Data API          |
| (ETL binary)      |                  +------------------------------+
|                   |
|  clean + toon     |      HTTPS       +------------------------------+
|  serialize/upload | ---------------> | Hugging Face Dataset API     |
+-------------------+                  +------------------------------+

+-------------------+      gRPC/HTTP2  +------------------------------+
| grpc_client       | ---------------> | grpc_server (tonic)          |
| (Cron caller)     |                  | Predict / PredictStream      |
|                   |                  | (mock inference at present)  |
+-------------------+                  +------------------------------+
         |
         | HTTPS (on alert)
         v
+-------------------+
| Telegram Bot API  |
+-------------------+
```

## Code-Level Structure

- `src/data_engine/main.rs`
  - ETL entrypoint.
  - Calls `fetcher`, `cleaner`, `toon` modules.
- `src/data_engine/fetcher.rs`
  - Pulls raw records from gov endpoint via `reqwest`.
- `src/data_engine/cleaner.rs`
  - Filters malformed/non-numeric values.
- `src/data_engine/toon.rs`
  - Serializes validated records to Toon format and uploads to HF.
- `src/main.rs`
  - gRPC server entrypoint (`grpc_server` binary).
- `src/inference_handler.rs`
  - Implements `InferenceService` RPC methods.
- `src/grpc_client.rs`
  - gRPC caller + threshold-based Telegram alert path.
- `proto/inference.proto`
  - Contract for request/response + streaming RPC.

## Data Contracts

### Raw record (gov fetch)
The fetcher expects each record to parse into:
- `id: String`
- `name: String`
- `value: serde_json::Value`

### Clean record (internal)
Cleaner normalizes into:
- `id: String`
- `name: String`
- `value: f64` (`NaN` and non-numeric dropped)

### Toon binary format (current)
Written by `serialize(records)` in `toon.rs`:
- 4 bytes magic: `TOON`
- 1 byte version: `1`
- 4 bytes LE record count (`u32`)
- newline-delimited JSON payload (one clean record per line)

### gRPC contract
`InferenceService` defines:
- `Predict(PredictRequest) -> PredictResponse`
- `PredictStream(PredictRequest) -> stream PredictResponse`

## Operational Boundaries

- External dependencies:
  - Taiwan gov data endpoint
  - Hugging Face dataset API
  - Telegram bot API
- Authentication points:
  - `HF_TOKEN` bearer token for upload
  - optional `GOV_API_KEY` header
  - `GRPC_AUTH_TOKEN` currently passed in request payload metadata map
- Server bind:
  - `grpc_server` listens on `[::]:50051`

## Known Gaps (Current State)

- Inference path uses mock confidence and does not load model weights yet.
- Request auth is not enforced on server side.
- Some HTTP paths do not currently enforce strict status checks.
