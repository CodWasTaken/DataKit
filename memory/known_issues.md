# Known issues

- `convert` with `--indent` currently ignores the value and always uses 2-space indent. Custom indentation will be implemented with a custom serializer.
- No format auto-detection — must specify output path or rely on extension convention.
- Large JSON files are read entirely into memory before processing.
