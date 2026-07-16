# DataKit

A production-quality command-line toolkit for inspecting, converting, validating, querying, transforming, and analyzing structured data.

## Features

- **Inspect** — Print a structural schema summary of any supported data file
- **Convert** — Transform data between supported formats
- (More commands planned — see [ROADMAP.md](ROADMAP.md))

## Supported formats

| Format  | Read | Write |
|---------|------|-------|
| JSON    | ✓    | ✓     |
| JSONL   | —    | —     |
| CSV     | —    | —     |
| TOML    | —    | —     |
| YAML    | —    | —     |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+

### Installation

```bash
git clone https://github.com/username/datakit.git
cd datakit
cargo build --release
```

### Usage

```bash
# Inspect a JSON file's structure
datakit inspect data.json

# Convert a JSON file (pretty-print)
datakit convert input.json output.json

# Convert from stdin to stdout
echo '{"hello":"world"}' | datakit convert -
```

## Project Structure

```
.
├── src/          # Source code
│   ├── main.rs   # Entry point
│   ├── cli.rs    # CLI argument definitions
│   ├── error.rs  # Structured error types
│   ├── inspect.rs# Inspect command
│   └── convert.rs# Convert command
├── tests/        # Integration tests
├── tasks/        # Task tracking
├── memory/       # Project knowledge
├── docs/         # Documentation
└── examples/     # Usage examples
```

## Development

```bash
make build      # Build the project
make test       # Run all tests
make lint       # Run clippy
make fmt        # Format code
make ci         # Full CI pipeline
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
