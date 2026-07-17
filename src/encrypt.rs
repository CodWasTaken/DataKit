use std::fs;

use argon2::Argon2;
use chacha20poly1305::AeadInPlace;
use chacha20poly1305::{Key, KeyInit, XChaCha20Poly1305, XNonce};
use rand::RngCore;

use crate::atomic;
use crate::cli::EncryptArgs;
use crate::error::Error;

pub fn run(args: EncryptArgs) -> Result<(), Error> {
    let plaintext = fs::read(&args.data)?;
    let password = if args.password_stdin {
        crate::secret::read_password_stdin()?
    } else {
        crate::secret::read_password("encryption password: ")?
    };

    let salt = random_bytes(32);
    let nonce = random_bytes(24);

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| Error::Message(format!("key derivation failed: {e}")))?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));
    let xnonce = XNonce::from_slice(&nonce);

    let mut ciphertext = plaintext.clone();
    let tag = cipher
        .encrypt_in_place_detached(xnonce, &[], &mut ciphertext)
        .map_err(|e| Error::Message(format!("encryption failed: {e}")))?;

    let aad = build_header(&salt, &nonce);
    let mut envelope = aad;
    envelope.extend_from_slice(&(ciphertext.len() as u64).to_le_bytes());
    envelope.extend_from_slice(&ciphertext);
    envelope.extend_from_slice(&tag);

    let output_path = args.output.unwrap_or_else(|| format!("{}.enc", args.data));
    atomic::write(&output_path, &envelope)?;
    println!("encrypted: {output_path}");

    zeroize_key(&mut key);
    Ok(())
}

fn build_header(salt: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut h = Vec::new();
    h.extend_from_slice(b"DKENCv1");
    h.push(1);
    h.extend_from_slice(&1u16.to_le_bytes());
    h.extend_from_slice(&1u16.to_le_bytes());
    h.extend_from_slice(&3u32.to_le_bytes());
    h.extend_from_slice(&(19456u32).to_le_bytes());
    h.extend_from_slice(&4u32.to_le_bytes());
    h.extend_from_slice(&(salt.len() as u16).to_le_bytes());
    h.extend_from_slice(salt);
    h.extend_from_slice(&(nonce.len() as u16).to_le_bytes());
    h.extend_from_slice(nonce);
    h
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut buf = vec![0u8; n];
    rand::rngs::OsRng.fill_bytes(&mut buf);
    buf
}

fn zeroize_key(key: &mut [u8; 32]) {
    use zeroize::Zeroize;
    key.zeroize();
}
