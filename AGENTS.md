# Agents

This file documents conventions and instructions for autonomous development agents working on DataKit.

## Workflow

- Read ROADMAP.md, tasks/ files, and memory/ before starting.
- Make one focused change per commit.
- Always run `cargo fmt`, `cargo clippy`, and `cargo test` before committing.
- Keep the repository in a working state after every commit.

## Conventions

- Format modules live under `src/format/`.
- Commands live under `src/` with a single file per command.
- Shared internal representation is `serde_json::Value`.
- Error type: `thiserror`-derived enum in `src/error.rs`.
- CLI parsing: `clap` derive macros.
- Tests: integration tests in `tests/`, CLI tests use `assert_cmd`.
