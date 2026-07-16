# Roadmap

## Phase 1 — Core CLI (current)

- [x] Rust CLI skeleton with clap
- [x] Structured error handling
- [x] JSON input support
- [x] `inspect` command — print schema summary
- [x] `convert` command — pretty-print or minify JSON
- [x] `validate` command — validate JSON against a schema
- [ ] `query` command — filter and extract values
- [ ] Support reading from stdin
- [ ] Support writing to stdout

## Phase 2 — Additional formats

- [x] JSON Lines (.jsonl) support
- [x] CSV support
- [ ] TOML support
- [ ] YAML support
- [ ] Format auto-detection from extension

## Phase 3 — Transformation & analysis

- [ ] `transform` command — apply jq-like filters
- [ ] `select` command — pick specific fields
- [ ] `filter` command — row-based filtering
- [ ] `stats` command — summary statistics
- [ ] `diff` command — structural diff

## Phase 4 — Polish & reliability

- [ ] Property-based testing for round-trips
- [ ] Fuzz testing for parsers
- [ ] Comprehensive error messages with source snippets
- [ ] Shell completions
- [ ] Man page
