# Toon Format Specification

## Purpose
Toon is the canonical serialized dataset format used by TGD-AAP data pipelines.
This document defines the binary framing contract, versioning policy, and compatibility rules.

## Binary Frame
A Toon payload is encoded as:

1. `magic` (4 bytes, ASCII): `TOON`
2. `version` (1 byte, unsigned integer)
3. `payload` (N bytes): UTF-8 JSON array of cleaned records

### Header constants
- Magic bytes: `0x54 0x4F 0x4F 0x4E` (`TOON`)
- Current version: `1`

## Versioning
Toon uses a single-byte format version with semantic intent:

- Patch-compatible adjustments do **not** change the version.
- Backward-compatible additive wire changes increment the minor protocol generation, represented by incrementing the version byte when required by decoder logic.
- Breaking wire changes must increment the version byte and be documented with migration notes.

Current defined versions:

- `v1`:
  - Header: `TOON` + `0x01`
  - Payload: JSON array of cleaned records
  - Record shape:
    - `id: string` (non-empty)
    - `name: string` (non-empty)
    - `value: number` (finite)

## Compatibility Rules

### Decoder requirements
- Decoder MUST reject payloads whose first 4 bytes are not `TOON`.
- Decoder MUST reject payloads with unsupported version bytes.
- Decoder MUST reject payloads that cannot be parsed as UTF-8 JSON.
- Decoder MUST reject payloads whose top-level JSON is not an array.

### Producer requirements
- Producer MUST emit exact magic header bytes `TOON`.
- Producer MUST emit a declared version supported by the deployment.
- Producer MUST serialize only schema-valid cleaned records.

## Deterministic rejection behavior
Any record violating schema constraints must be rejected deterministically and must not be silently repaired.
At minimum, these malformed forms are rejected:

- Empty `id`
- Empty `name`
- `value` not representable as finite numeric value (including `null`, object, array, boolean, non-numeric string, `NaN`, or infinity)

## Forward evolution guidance
- Reserve new version bytes for wire-level changes.
- Keep old decoders strict by default; do not auto-downgrade unknown versions.
- If multi-version support is needed, dispatch by header version first, then parse with version-specific schema logic.
