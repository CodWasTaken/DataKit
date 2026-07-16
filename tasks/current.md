# Current task

## TC-002 — Add JSON Lines (.jsonl) format support

**Status:** In progress

**Goal:** Support reading and writing JSON Lines format, auto-detected by `.jsonl` file extension.

**Implementation:**
- `src/format/mod.rs` — format detection and Format enum
- `src/format/jsonl.rs` — JSONL reader/writer
- Updated `inspect.rs` — shows record count and first-record schema for JSONL
- Updated `convert.rs` — supports JSON↔JSONL conversion matrix
- Added 7 integration tests

**Acceptance criteria:**
- `datakit inspect data.jsonl` shows record count and schema
- `datakit convert data.jsonl output.json` converts JSONL to JSON array
- `datakit convert data.json output.jsonl` converts JSON array to JSONL
- `datakit convert data.jsonl output.jsonl` copies JSONL identically
- Error on non-array JSON → JSONL
- Error on invalid JSONL line with line number
- All existing tests still pass
