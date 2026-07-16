# Conventions

## Code style

- 2-space indent (matching .editorconfig)
- `cargo fmt` formatting
- No `unsafe` code
- No wildcard imports (`use foo::*`)

## Error handling

- Use `thiserror` for error type definitions
- Return `Result<_, Error>` from all command entry points
- Use `?` for propagation
- Construct user-facing errors with `Error::Message`

## CLI

- Use `clap` derive API
- Commands are subcommands of `datakit`
- File arguments are positional; `-` means stdin
- Global `--help` and per-command `--help` are automatic

## Testing

- Integration tests in `tests/` directory
- Use `assert_cmd` for CLI tests
- Use `tempfile` for temporary directories
- Test stdin, stdout, and file I/O paths
- Test error cases (missing file, invalid JSON)

## Imports

- External crates first, blank line, then `crate::` imports
- Group by module when multiple `crate::` imports exist
