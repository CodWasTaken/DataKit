# DataKit

A production-quality command-line toolkit for inspecting, converting, validating, querying, transforming, and analyzing structured data.

## Supported formats

| Format  | Read | Write |
|---------|------|-------|
| JSON    | ✓    | ✓     |
| JSONL   | ✓    | ✓     |
| CSV     | ✓    | ✓     |
| TOML    | ✓    | ✓     |
| YAML    | ✓    | ✓     |

## Commands

### Inspection & analysis
- **`inspect`** — Print a structural schema summary of any data file
- **`stats`** — Compute summary statistics for numeric fields
- **`count`** — Count records in a data file
- **`unique`** — List unique values of a field
- **`diff`** — Show line-level differences between two data files

### Extraction & filtering
- **`query`** — Extract a value by field path (`user.name`, `items[0].id`)
- **`select`** — Pick specific fields from records
- **`filter`** — Filter records by condition (`age > 30`)
- **`slice`** — Extract a range of array records

### Transformation
- **`convert`** — Transform data between any supported formats
- **`sort`** — Sort records by a field
- **`reverse`** — Reverse array order
- **`rename`** — Rename a field
- **`flatten`** — Flatten nested objects into dot-separated keys
- **`fill`** — Replace null values in a field
- **`explode`** — Expand array fields into multiple records
- **`dedup`** — Remove duplicate records
- **`round`** — Round numeric values to a given precision
- **`merge`** — Combine two objects or arrays
- **`zip`** — Zip two arrays into pairs
- **`entries`** — Convert object to key-value entry records
- **`keys`** — List keys of an object
- **`values`** — List values of an object

### Randomization
- **`shuffle`** — Randomly reorder array records
- **`sample`** — Random sample of records
- **`pick`** — Pick a single random record

### Validation
- **`validate`** — Validate data against a JSON Schema

### Shell integration
- **`completions`** — Generate shell completions (bash, zsh, fish, elvish, powershell)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+

### Installation

```bash
git clone https://github.com/CodWasTaken/DataKit.git
cd DataKit
cargo build --release
```

### Usage examples

```bash
# Inspect structure
datakit inspect data.json
datakit inspect data.jsonl
datakit inspect data.csv
datakit inspect config.toml
datakit inspect config.yaml

# Convert between formats
datakit convert data.jsonl data.json
datakit convert data.json data.csv
datakit convert data.json data.toml
datakit convert data.json data.yaml
datakit convert input.json output.json --indent 4
datakit convert input.dat output.dat --from json --to yaml

# Validate against JSON Schema
datakit validate data.json --schema schema.json

# Query fields
datakit query data.json --path "user.name"
datakit query data.json --path "items[0].id"

# Filter records
datakit filter data.json --condition "age > 30"

# Statistics
datakit stats data.json

# Sort
datakit sort data.json --by "name"
datakit sort data.json --by "age" --desc

# Transform
datakit flatten data.json
datakit rename data.json --mapping "old_name:new_name"
datakit fill data.json --field "email" --value "unknown@example.com"
datakit explode data.json --field "items"
datakit dedup data.json --field "id"
datakit round data.json --decimals 2
datakit merge a.json b.json

# Randomization
datakit shuffle data.json --seed 42
datakit sample data.json --count 5 --seed 42
datakit pick data.json --seed 42

# Shell completions
datakit completions bash > /etc/bash_completion.d/datakit

# stdin/stdout pipeline
echo '{"hello":"world"}' | datakit convert -
```

## Project Structure

```
.
├── src/           # Rust source (33 modules)
│   ├── main.rs    # Entry point
│   ├── cli.rs     # CLI argument definitions
│   ├── error.rs   # Structured error types
│   ├── format/    # Format-specific parsers/serializers
│   │   ├── csv.rs
│   │   ├── jsonl.rs
│   │   ├── toml.rs
│   │   └── yaml.rs
│   ├── inspect.rs
│   ├── convert.rs
│   ├── validate.rs
│   ├── query.rs
│   ├── stats.rs
│   ├── filter.rs
│   ├── select.rs
│   ├── sort.rs
│   ├── diff.rs
│   ├── count.rs
│   ├── unique.rs
│   ├── flatten.rs
│   ├── rename.rs
│   ├── fill.rs
│   ├── explode.rs
│   ├── dedup.rs
│   ├── round.rs
│   ├── merge.rs
│   ├── zip.rs
│   ├── entries.rs
│   ├── keys.rs
│   ├── values.rs
│   ├── slice.rs
│   ├── head.rs
│   ├── tail.rs
│   ├── reverse.rs
│   ├── shuffle.rs
│   ├── sample.rs
│   ├── pick.rs
│   └── completions.rs
├── tests/         # Integration tests (89+ tests)
├── memory/        # Project knowledge
├── tasks/         # Task tracking
├── docs/          # Documentation
└── examples/      # Usage examples
```

## Development

```bash
make build      # cargo build
make test       # cargo test --all-features
make lint       # cargo clippy -- -D warnings
make fmt        # cargo fmt
make ci         # fmt + clippy + test
```

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned work.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
