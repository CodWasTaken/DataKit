# DataKit Threat Model

## Overview

DataKit is a command-line toolkit for processing structured data. This document defines the security threat model for cryptographic operations.

## Trust Assumptions

- The user has legitimate access to the data they process.
- The local filesystem and operating system are trusted.
- The user's terminal is trusted for password entry.
- The attacker can read, modify, or delete any file the user can access.
- The attacker may observe the network.
- The attacker does not control the user's local machine while a cryptographic operation is running.

## Security Goals

| Goal | Priority | Description |
|------|----------|-------------|
| Confidentiality | high | Plaintext is not recoverable from ciphertext without the correct key. |
| Integrity | high | Any modification of ciphertext or envelope is detected before decryption. |
| Authentication | high | The decryptor confirms that the ciphertext was created by someone who knew the correct password/key. |
| Non-repudiation | medium | Signed data can be attributed to a specific public key. |
| Forward secrecy | low | No goal — we do not implement interactive protocols. |

## Attacker Capabilities

| Capability | Applies to | Notes |
|------------|-----------|-------|
| Read ciphertext files | Encryption | Standard threat. Solved by AEAD. |
| Modify ciphertext files | Encryption | Detected by AEAD authentication tag. |
| Truncate ciphertext | Encryption | Detected by incomplete tag failure. |
| Modify envelope header | Encryption | Authenticated header prevents this. |
| Observe password entry | Encryption | Mitigated by no-echo terminal input. |
| Read key files | Signatures | Mitigated by file permissions (0600). |
| Modify key files | Signatures | Detected by key fingerprint verification. |
| Replay old ciphertexts | Encryption | Not currently addressed — may add key rotation metadata. |
| Side-channel attacks | All | Not modeled — we rely on library implementations. |

## Algorithm Selection

| Algorithm | Purpose | Rationale |
|-----------|---------|-----------|
| XChaCha20-Poly1305 | Authenticated encryption | Misuse-resistant, no padding oracle, constant-time in Rust implementation. |
| Argon2id | Password-based KDF | Memory-hard, resists GPU/ASIC attacks, recommended by OWASP. |
| Ed25519 | Digital signatures | Fast, small keys, well-audited implementations. |
| BLAKE3 | General hashing | Fast, parallel, suitable for checksums. |

## Non-Goals

- Protection against attackers with physical access to the running machine.
- Anonymous file sharing.
- Protection against attackers who can intercept the user's terminal hardware.
- Formal verification of cryptographic implementations.

## Dependency Audit

| Crate | Purpose | Status |
|-------|---------|--------|
| `chacha20poly1305` | AEAD | RustCrypto, well-maintained, reviewed. |
| `argon2` | KDF | RustCrypto, well-maintained. |
| `ed25519-dalek` | Signatures | Well-audited, used by Signal. |
| `blake3` | Hashing | BLAKE3 team, optimized, audited. |

## Operational Security

- Passwords are never logged, echoed, or stored in command history.
- Key files are created with 0600 permissions.
- Temporary files are created with restricted permissions and atomically renamed.
- Detailed error messages avoid distinguishing between wrong password and modified ciphertext.

## Future Considerations

- Key rotation metadata in envelope
- Hardware-backed key storage
- Multi-recipient encryption
- Plausible deniability formats
