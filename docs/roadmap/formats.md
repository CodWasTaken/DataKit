# Formats Roadmap

Goal: Support every commonly used structured-data format.

## Legend
- ✅ Read + Write
- ◐ Read only
- ❌ Not implemented

| Format | Status | Priority |
|--------|--------|----------|
| JSON | ✅ | done |
| JSON Lines | ✅ | done |
| CSV | ✅ | done |
| TOML | ✅ | done |
| YAML | ✅ | done |
| XML | ❌ | high |
| MessagePack | ❌ | medium |
| CBOR | ❌ | medium |
| INI | ❌ | low |
| env files | ❌ | low |
| Markdown tables | ❌ | low |

## Standards

Every format implementation must include:
- Format detection from extension
- read() → Value
- write(Value) → bytes
- Invalid-input diagnostics
- Edge-case tests (empty, deeply nested, duplicate keys, etc.)
- Round-trip tests where lossless
- CLI level tests (inspect + convert + pipe)
