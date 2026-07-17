# Agents

This file documents conventions and instructions for autonomous development agents working on DataKit.

## Workflow

- Read ROADMAP.md, tasks/ files, memory/, and docs/roadmap/ before starting.
- Make one focused change per commit.
- Always run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test --all-features` before committing.
- Keep the repository in a working state after every commit.
- Push to GitHub after every commit.

## Conventions

- Format modules live under `src/format/`.
- Commands live under `src/` with a single file per command.
- Shared internal representation is `serde_json::Value`.
- Error type: `thiserror`-derived enum in `src/error.rs`.
- CLI parsing: `clap` derive macros.
- Tests: integration tests in `tests/`, CLI tests use `assert_cmd`.
- Centralized atomic file writes: `atomic::write_output()`.
- All commands support `-` for stdin.
- Separate CLI handlers from domain logic.
- Core logic testable without spawning CLI.

## Domains

| Dir | Purpose |
|-----|---------|
| `src/format/` | Format-specific parsers/serializers |
| `src/*.rs` | One file per command |
| `memory/` | Durable project knowledge |
| `tasks/` | Task tracking |
| `docs/roadmap/` | Subsystem roadmaps |
| `docs/security/` | Security documentation |

## Cryptography rules

- Never implement a cryptographic primitive from scratch.
- Use established, maintained Rust cryptography libraries.
- Default to authenticated encryption (XChaCha20-Poly1305).
- Never reuse a nonce with the same key.
- Derive keys using password-based KDF (Argon2).
- Zeroize sensitive material where practical.
- Never expose unauthenticated encryption as a secure option.
- Never call encoding or hashing "encryption."
- Emit insecurity warnings for MD5, SHA-1.
