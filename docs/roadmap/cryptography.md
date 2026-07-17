# Cryptography Roadmap

Goal: Safe, misuse-resistant, documented cryptographic operations.

## Principles
- Never implement primitives from scratch
- Use established audited Rust crypto libraries
- Prefer misuse-resistant authenticated constructions
- Default to XChaCha20-Poly1305
- Never call encoding encryption
- Never call hashing encryption
- Never imply checksums provide authenticity

## Order
1. Write threat model
2. Specify encrypted envelope v1
3. Secure password input
4. Authenticated encryption (encrypt)
5. Authenticated decryption (decrypt)
6. Integrity tests for all failure modes
7. Key generation (Ed25519)
8. Key file permissions
9. Detached signatures (sign + verify)

## Dependencies
- `chacha20poly1305` — authenticated encryption
- `argon2` — password-based key derivation
- `ed25519-dalek` — signatures
- `zeroize` — memory clearing
