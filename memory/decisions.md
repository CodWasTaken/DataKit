# Decisions

## 2026-07-16: Use serde_json::Value as internal representation

**Context:** DataKit needs a shared internal representation that works across formats.

**Decision:** Use `serde_json::Value` directly. It is well-tested, widely used, handles both document and tabular data, and avoids introducing a custom IR.

**Consequences:** CSV and JSONL must convert rows into arrays of objects. This is a natural mapping.

## 2026-07-16: Use clap derive API

**Context:** CLI argument parsing needs to be maintainable and well-typed.

**Decision:** Use clap's derive macros rather than the builder API.

**Consequences:** Less boilerplate, automatic help generation, compile-time validation of argument definitions.

## 2026-07-16: Use thiserror for error types

**Context:** Error handling needs to be structured without a heavy framework.

**Decision:** Use `thiserror` to derive `Display` and `Error` on a custom error enum.

**Consequences:** Clean error propagation with `?`, easy to extend with new variants, no external error-reporting runtime dependency for now.
