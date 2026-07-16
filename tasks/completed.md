# Completed tasks

## TC-006 — Add TOML format support

**Completed:** 2026-07-16

**Commit:** (pending)

**Deliverables:**
- TOML reader via `toml` crate (parses to `serde_json::Value`)
- TOML writer (requires top-level object)
- TOML detect_format via `.toml` extension
- TOML integration in inspect and convert
- 4 new integration tests (44 total, all passing)
- `toml` crate dependency

## TC-005 — Add query command for field path extraction

**Completed:** 2026-07-16

**Commit:** ebe62d3

**Deliverables:**
- New `query` subcommand with `--path` option
- Dot-separated path syntax: `field`, `field.subfield`, `field[0]`, `field[0].sub`
- Works with any supported input format (JSON, JSONL, CSV)
- Clear error messages for missing keys, out-of-bounds indices, type mismatches
- 6 new integration tests (40 total, all passing)

## TC-004 — Add JSON Schema validation command

**Completed:** 2026-07-16

**Commit:** 182364c

**Deliverables:**
- New `validate` subcommand with `--schema` flag
- Uses `jsonschema` crate for JSON Schema Draft validation
- Reads data in any supported format (JSON, JSONL, CSV)
- Reports all validation errors with instance paths
- Exit code 1 on validation failure
- 4 new integration tests (34 total, all passing)
- `jsonschema` crate dependency

## TC-003 — Add CSV format support

**Completed:** 2026-07-16

**Commit:** b7d58de

**Deliverables:**
- CSV reader with header-based object mapping
- CSV writer with automatic header collection
- Type inference for CSV values (number, boolean, null, string)
- CSV-inspect (row count, column types)
- Full conversion matrix (CSV ↔ JSON, CSV ↔ JSONL, CSV ↔ CSV roundtrip)
- 7 new integration tests (30 total, all passing)
- `csv` crate dependency

## TC-002 — Add JSON Lines (.jsonl) format support

**Completed:** 2026-07-16

**Commit:** eb26c1e

**Deliverables:**
- Format detection via `.jsonl` extension
- JSONL reader (skips empty/comment lines, reports line numbers on error)
- JSONL writer (compact, one value per line)
- JSON ↔ JSONL conversion matrix in `convert` command
- JSONL-aware `inspect` command (record count + first record schema)
- 7 new integration tests (23 total, all passing)

## TC-001 — Bootstrap DataKit Rust CLI skeleton

**Completed:** 2026-07-16

**Commit:** 91a7de7

**Deliverables:**
- Rust CLI project with clap argument parsing
- `inspect` command — prints JSON schema summary
- `convert` command — pretty-print JSON
- Structured error handling with thiserror
- 16 integration tests covering inspect, convert, error cases
- CI workflow with fmt, clippy, test
- Makefile with common commands
- Project documentation (README, ROADMAP, AGENTS)
- Memory files (architecture, conventions, decisions, known issues)
- Task tracking (current, backlog, blocked, completed)
