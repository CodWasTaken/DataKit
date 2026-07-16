# Current task

## TC-001 — Bootstrap DataKit Rust CLI skeleton

**Status:** In progress

**Goal:** Create the initial Rust CLI project with inspect and convert commands supporting JSON.

**Implementation plan:**

1. Create Cargo.toml with clap, serde_json, thiserror
2. Create src/main.rs with command dispatch
3. Create src/cli.rs with Clap argument definitions
4. Create src/error.rs with structured error types
5. Create src/inspect.rs — read JSON, print schema summary
6. Create src/convert.rs — read and re-serialize JSON
7. Create tests/cli_tests.rs — integration tests
8. Update Makefile with Rust commands
9. Create .github/workflows/ci.yml
10. Update README.md with project documentation
11. Run cargo fmt, clippy, test
12. Commit

**Acceptance criteria:**

- `cargo build` succeeds
- `cargo test --all-features` passes all tests
- `cargo clippy --all-targets --all-features -- -D warnings` passes
- `cargo fmt --all -- --check` passes
- `datakit inspect file.json` prints schema info
- `datakit convert input.json output.json` produces valid JSON
- `datakit convert input.json` prints pretty-printed JSON to stdout
