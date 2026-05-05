# Runbook: Inference (`grpc_server` + `grpc_client`)

## Purpose
Operate and validate gRPC inference serving and client-triggered alert flow.

## Binaries
- Server: `cargo run --bin grpc_server`
- Client: `cargo run --bin grpc_client`

## Current Behavior Note
`Predict` currently returns mock confidence (`0.42`) from `inference_handler.rs`. This is expected in current repo state.

## Server Startup

```bash
cargo run --bin grpc_server
```

Healthy signal:
- `Starting gRPC server on [::]:50051`

## Client Invocation

```bash
export GRPC_SERVER_URL="http://[::1]:50051"
export GRPC_AUTH_TOKEN="<optional>"
export ALERT_THRESHOLD="0.8"

# required only if alert path is expected
export TELEGRAM_BOT_TOKEN="<token>"
export TELEGRAM_CHAT_ID="<chat-id>"

cargo run --bin grpc_client
```

Healthy signal:
- `Connecting to gRPC server at <url>`
- `Prediction received: confidence=0.4200, alert=false`

## Alerting Path
Client sends Telegram message when either condition is true:
- `response.alert_triggered == true`
- `response.confidence >= ALERT_THRESHOLD`

Message format:
- `[TGD-AAP Alert] <response message>`

## Failure Modes and Triage

1. `GRPC_SERVER_URL env var not set`
- Action: export `GRPC_SERVER_URL`.

2. Connection failure to server
- Action: verify server process is running and URL scheme is `http://`.

3. Telegram secret missing when alert condition met
- Errors: missing `TELEGRAM_BOT_TOKEN` or `TELEGRAM_CHAT_ID`.
- Action: set required secrets.

4. Auth token expectations mismatch
- Note: token is currently sent in request metadata map payload, not transport metadata interceptor.
- Action: align client/server auth strategy before enforcing auth in production.

## Operational Checks
- Confirm request IDs appear in server logs (`Predict called: request_id=...`).
- Confirm client receives response and exits successfully.
