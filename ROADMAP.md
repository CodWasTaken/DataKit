# DataKit Roadmap

Product-level phases and subsystem plans. Detailed per-subsystem roadmaps live in `docs/roadmap/`.

## Phase 1 — Core CLI (complete)

- [x] Rust CLI skeleton with clap derive
- [x] Structured error handling (thiserror)
- [x] Shared internal representation (serde_json::Value)
- [x] Standard input and output (- convention)
- [x] Centralized atomic file writes (tempfile + rename)
- [x] `inspect` command — schema summary
- [x] `convert` command — format transformation
- [x] `validate` command — JSON Schema validation
- [x] `query` command — field path extraction
- [x] `completions` command — shell completions
- [x] CI pipeline (fmt, clippy, test on push/PR)

## Phase 2 — Formats (JSON/JSONL/CSV/TOML/YAML complete)

- [x] JSON (read + write)
- [x] JSON Lines (read + write)
- [x] CSV with type inference (read + write)
- [x] TOML (read + write)
- [x] YAML (read + write)
- [x] Format auto-detection from file extension
- [x] Explicit --from / --to format flags
- [ ] XML (read + write)
- [ ] MessagePack / CBOR (read + write)
- [ ] INI (read + write)
- [ ] Environment file support
- [ ] Markdown tables (read)

## Phase 3 — Transformation commands (26/30+ complete)

- [x] select, filter, sort, reverse, rename, flatten, fill, explode
- [x] dedup, merge, zip, entries, keys, values, round
- [x] slice, head, tail, sample, shuffle, pick
- [x] search, length, pretty, check, diff
- [ ] group, aggregate, pivot, unpivot, transpose, normalize
- [ ] transform — jq-like expression filters

## Phase 4 — Statistics and profiling (basic complete)

- [x] `stats` command — count, min, max, mean, median, stddev
- [x] `count` command — record count
- [x] `unique` command — unique field values
- [ ] Histograms, quantiles, correlations
- [ ] String-length distributions
- [ ] Missingness patterns
- [ ] Streaming statistics

## Phase 5 — Encoding and decoding

- [x] hex encode/decode
- [x] base64 encode/decode
- [ ] base64url encode/decode
- [ ] base32, base32hex encode/decode
- [ ] base58 encode/decode
- [ ] base85 encode/decode
- [ ] Property-based round-trip tests
- [ ] Strict and permissive decode modes

## Phase 6 — Hashing and checksums

- [x] MD5 hashing (insecure, warning emitted)
- [ ] SHA-256, SHA-512
- [ ] BLAKE3
- [ ] SHA-3 variants
- [ ] Checksum manifest creation (--manifest)
- [ ] Checksum manifest verification (--check)
- [ ] Recursive directory hashing
- [ ] Machine-readable output

## Phase 7 — Cryptography and security

- [ ] Threat model document (docs/security/)
- [ ] Encrypted envelope specification (docs/security/)
- [ ] Secure password input module
- [ ] Authenticated encryption (XChaCha20-Poly1305)
- [ ] Decrypt with full verification
- [ ] Wrong-password / truncation / tamper tests
- [ ] Key generation (Ed25519)
- [ ] Key file permissions
- [ ] Detached signatures (Ed25519)
- [ ] Signature verification

## Phase 8 — Compression and archives

- [ ] gzip compression
- [ ] zstd compression
- [ ] deflate, brotli, lz4, xz, bzip2
- [ ] Archive listing (tar, zip)
- [ ] Archive integrity testing
- [ ] Safe archive extraction (path traversal protection)
- [ ] Archive creation

## Phase 9 — Privacy and redaction

- [ ] Field removal
- [ ] Fixed replacement
- [ ] Partial masking
- [ ] Deterministic keyed pseudonyms
- [ ] Secret/email/phone/IP detection
- [ ] Value shuffling

## Phase 10 — Synthetic data generation

- [ ] Primitive value generators
- [ ] UUID generation
- [ ] Date/time generation
- [ ] Schema-conforming generation
- [ ] Tabular record generation
- [ ] Configurable distributions
- [ ] Invalid/boundary fixtures

## Phase 11 — Binary and text operations

- [ ] Hex dump
- [ ] Byte slicing and search
- [ ] Magic-number inspection
- [ ] String extraction
- [ ] UTF-8 validation
- [ ] Unicode normalization
- [ ] Newline conversion
- [ ] Regex filtering and replacement

## Phase 12 — Declarative pipelines

- [ ] Pipeline specification (YAML)
- [ ] Pipeline validation
- [ ] Pipeline execution engine
- [ ] Deterministic execution
- [ ] Dry-run mode
- [ ] Atomic output per step
- [ ] Full integration tests

## Phase 13 — Performance and scalability

- [ ] Streaming large-file support
- [ ] Bounded-memory operations
- [ ] Benchmarks for each subsystem
- [ ] Parallel operations where deterministic
- [ ] Fuzz testing for all parsers

## Phase 14 — Packaging and documentation

- [ ] Per-command documentation
- [ ] Format matrix documentation
- [ ] Algorithm matrix documentation
- [ ] Security model documentation
- [ ] Encrypted envelope specification
- [ ] Exit-code reference
- [ ] Compatibility policy
- [ ] Examples cookbook
- [ ] Man page
- [ ] Homebrew formula
- [ ] Docker image
