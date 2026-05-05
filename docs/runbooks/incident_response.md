# Runbook: Incident Response

## Scope
Response procedure for failures across ETL, gRPC inference serving, and alert delivery.

## Severity Levels
- SEV-1: End-to-end pipeline unavailable (no ETL and no inference path).
- SEV-2: One major path unavailable (ETL or inference/alerting).
- SEV-3: Degraded behavior (partial data quality issues, intermittent external API failures).

## First 15 Minutes

1. Identify failing component
- `data_engine` (ingest/upload)
- `grpc_server` (serve)
- `grpc_client` (invoke/alert)

2. Capture immediate evidence
- Last command run
- Full stderr/stdout logs
- Relevant env vars present/missing (do not expose secret values)

3. Stabilize
- If server down: restart `grpc_server`.
- If client failing: test connectivity against running server.
- If ETL failing on external API: confirm endpoint availability and retry window.

## Diagnostic Commands

```bash
# Build sanity
cargo build

# ETL path
cargo run --bin data_engine

# Server path
cargo run --bin grpc_server

# Client path (with required env vars set)
cargo run --bin grpc_client
```

## Component-Specific Playbooks

### ETL Incident
- Check for `HF_TOKEN` presence and permissions.
- Validate `GOV_API_URL` reachability.
- Confirm run reaches `Dataset uploaded to Hugging Face — ETL complete`.

### Inference Server Incident
- Ensure no port conflict on `50051`.
- Confirm server startup log appears.
- Verify inbound request logs (`Predict called: request_id=...`).

### Alerting Incident
- Confirm threshold condition actually met.
- Validate `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID` presence.
- Verify Telegram API reachability.

## Communication Template
- Impact: what functionality is down/degraded.
- Start time: timestamp with timezone.
- Scope: ETL / inference / alerting.
- Mitigation in progress: restart, config correction, dependency check.
- Next update ETA: 15-30 minutes.

## Recovery Criteria
- ETL: successful full run with upload completion log.
- Inference: successful client call with response logged.
- Alerting: successful Telegram send on forced threshold test.

## Follow-up Actions (Post-Incident)
- Document root cause.
- Add reproducible test if gap is testable.
- Create reliability/security hardening item if failure exposed known gap.
