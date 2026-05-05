# Toon Format Specification

## Purpose
Toon is the canonical serialized dataset format used by TGD-AAP.

## Binary Frame
A Toon payload is encoded as:
1. `magic` (4 bytes, ASCII): `TOON`
2. `version` (1 byte, unsigned integer)
3. `record_count` (4 bytes, little-endian `u32`)
4. `payload` (N bytes): newline-delimited UTF-8 JSON objects (`jsonl`)

## Current Version
- `v1`:
  - Header: `TOON` + `0x01`
  - Count field: expected number of JSON lines
  - Record shape per line:
    - `id: string` (non-empty)
    - `name: string` (non-empty)
    - `value: number` (finite)

## Decoder Requirements
- Reject payloads shorter than 9 bytes.
- Reject payloads whose first 4 bytes are not `TOON`.
- Reject unsupported version bytes.
- Reject non-UTF-8 payload.
- Reject malformed JSON lines.
- Reject payloads where decoded line count does not match declared `record_count`.

## Producer Requirements
- Emit exact magic bytes `TOON`.
- Emit supported version.
- Emit deterministic `record_count` matching encoded records.
- Emit only schema-valid cleaned records.

## Versioning Guidance
- Increment version for breaking wire-format changes.
- Keep decoder strict by default for unknown versions.
- If multi-version support is added, dispatch by header version first.
