# DataKit Encrypted Envelope — Version 1

## Binary Format

All integers are little-endian. Variable-length fields are length-prefixed.

```
┌──────────────────────────────┐
│ Magic: "DKENCv1" (7 bytes)   │
├──────────────────────────────┤
│ Version: 0x01 (1 byte)       │
├──────────────────────────────┤
│ Ciphertext-Algo: uint16      │
│   0x0001 = XChaCha20-Poly1305│
├──────────────────────────────┤
│ KDF-Algo: uint16             │
│   0x0001 = Argon2id          │
├──────────────────────────────┤
│ KDF-Params: variable         │
│   Argon2id:                  │
│     Time-Cost: uint32        │
│     Mem-Cost-KiB: uint32     │
│     Parallelism: uint32      │
│     Salt-Len: uint16         │
│     Salt: [Salt-Len] bytes   │
├──────────────────────────────┤
│ Nonce-Len: uint16            │
│ Nonce: [Nonce-Len] bytes     │
├──────────────────────────────┤
│ AAD-Len: uint16              │
│ AAD: [AAD-Len] bytes         │
├──────────────────────────────┤
│ Ciphertext-Len: uint64       │
│ Ciphertext: [Ciphertext-Len] │
├──────────────────────────────┤
│ Tag: [16 bytes]              │
└──────────────────────────────┘
```

## Algorithm Identifiers

| ID | Algorithm |
|----|-----------|
| 0x0001 | XChaCha20-Poly1305 |
| 0x0002 | AES-256-GCM |

## KDF Identifiers

| ID | Algorithm |
|----|-----------|
| 0x0001 | Argon2id |

## Key Derivation

Password is processed by Argon2id with the stored parameters to produce a 32-byte symmetric key.

## Decryption Process

1. Read and validate magic + version.
2. Read algorithm identifiers.
3. Read KDF parameters and derive key from password.
4. Read nonce, AAD.
5. Read ciphertext length and ciphertext.
6. Read authentication tag.
7. Authenticate (AAD + ciphertext) with tag.
8. Decrypt and output plaintext only after authentication succeeds.

## Compatibility

- Version 1 is the initial format.
- Future versions must increment the version byte.
- Unknown mandatory algorithm identifiers must be rejected.
- The envelope authenticates its own header (AAD).
