# Encoding Roadmap

Goal: Complete encoding/decoding library with strict and permissive modes.

## Implemented
- hex (via `encode --algorithm hex`)
- base64 (via `encode --algorithm base64`)

## Planned
- Split into separate `hex`, `base64`, `base64url` commands
- base32, base32hex
- base58
- base85
- URL encoding
- Property-based round-trip tests for all
- Published test vectors for each
- Strict mode: reject malformed input, trailing data
- Permissive mode: ignore whitespace, continue on error
