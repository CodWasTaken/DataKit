# Architecture

## Overview

DataKit is a CLI toolkit for structured data. Each command reads data, processes it through a shared internal representation, and produces output.

## Internal representation

The shared internal representation is `serde_json::Value` — a recursive tree of null, bool, number, string, array, and object values. This is sufficiently expressive for both document formats (JSON, YAML, TOML) and tabular formats (CSV, JSONL).

## Module structure

```
src/
  main.rs      — Entry point, command dispatch
  cli.rs       — Clap CLI argument definitions
  error.rs     — Structured error types
  inspect.rs   — Inspect command: schema summary
  convert.rs   — Convert command: format transformation
  format/
    mod.rs     — Format detection, Format enum
    jsonl.rs   — JSON Lines reader/writer
    csv.rs     — CSV reader/writer
```

## CLI design

Every command follows the pattern: parse arguments → read input → process → write output.

Input comes from a file path or stdin. Output goes to a file path or stdout.

Error handling uses `thiserror` for typed errors and propagates via `Result<_, Error>`.
