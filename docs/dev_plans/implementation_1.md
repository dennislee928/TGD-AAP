
  - Architecture vision is strong.
  - Core binaries/workflows exist.
  - Several critical parts are still placeholders or lack reliability/security controls.

  ## Strengthening Plan (Prioritized)

  1. Foundation and truth alignment (Week 1)

  - Rewrite readme.md to separate:
      - Implemented now
      - Roadmap / planned
  - Add quickstart, env var matrix, local run commands, and expected outputs for:
      - data_engine
      - grpc_server
      - grpc_client
  - Add docs/architecture.md and docs/runbooks/ (ETL runbook, inference runbook, incident response).

  2. Reliability hardening (Week 1-2)

  - Add strict HTTP error handling in ETL and alert flows:
      - check status codes (error_for_status)
      - add request timeouts + retry/backoff
      - remove silent fallbacks (unwrap_or_default on API parse)
  - Make failures explicit in logs and exit codes for GitHub Actions.
  - Add circuit temp-file uniqueness (avoid shared /tmp/circuit.qasm collisions).

  3. Security and secret hygiene (Week 2)

  - Move auth token from request payload metadata into proper gRPC metadata/interceptor.
  - Add optional mTLS or token validation layer on server.
  - Add .env.example with non-secret placeholders and document secret scopes.
  - Add dependency and secret scanning in CI (cargo audit, gitleaks/equivalent).

  4. Data contract and schema quality (Week 2-3)

  - Formalize Toon spec in docs/toon-spec.md (header, versioning, compatibility policy).
  - Add deserialize + round-trip validation tests for Toon.
  - Add schema checks for incoming gov data and reject malformed payloads deterministically.

  5. Inference completeness (Week 3)

  - Replace mock confidence in src/inference_handler.rs with real inference pipeline:
      - model load
      - feature mapping
      - quantum-bridge integration path
  - Define clear fallback behavior if holyQASM/model is unavailable.

  6. Testing strategy (Week 3-4)

  - Add:
      - unit tests (cleaner, toon, proto conversion)
      - integration tests (gRPC server/client contract)
      - workflow smoke tests for both cron jobs
  - Introduce minimal fixture dataset under tests/fixtures/.
  - Gate PR merges on test pass + lint pass.

  7. CI/CD maturity (Week 4)

  - Add dedicated CI workflow:
      - cargo fmt --check
      - cargo clippy -D warnings
      - cargo test
      - security scan step
  - Improve cache key strategy and pin action versions by commit SHA for supply-chain safety.

  8. Observability and operations (Week 4-5)

  - Add structured log fields (request_id, dataset version, model version).
  - Add basic metrics (latency/error rate) and optional OpenTelemetry export.
  - Add alert dedup/cooldown to prevent Telegram spam bursts.

  ## Highest-impact gaps to address first

  - Mock inference still in use.
  - Silent error handling in external calls.
  - README mixes aspirational and implemented states.
  - No test coverage or CI quality gates.