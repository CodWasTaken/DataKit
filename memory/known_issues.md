# Known issues

- `convert` with `--indent` currently ignores the value and always uses 2-space indent. Custom indentation will be implemented with a custom serializer.
- Large files are read entirely into memory before processing.
- CSV type inference is heuristic — values like "123" are parsed as numbers, which may not be desired for fields like ZIP codes.
