use std::fs;

use argon2::Argon2;
use chacha20poly1305::AeadInPlace;
use chacha20poly1305::{Key, KeyInit, Tag, XChaCha20Poly1305, XNonce};

use crate::atomic;
use crate::cli::DecryptArgs;
use crate::error::Error;

pub fn run(args: DecryptArgs) -> Result<(), Error> {
    let envelope = fs::read(&args.data)?;

    if envelope.len() < 7 || &envelope[..7] != b"DKENCv1" {
        return Err(Error::Message(
            "not a valid DataKit encrypted file".to_string(),
        ));
    }
    if envelope[7] != 1 {
        return Err(Error::Message(format!(
            "unsupported envelope version {}",
            envelope[7]
        )));
    }

    let password = if args.password_stdin {
        crate::secret::read_password_stdin()?
    } else {
        crate::secret::read_password("decryption password: ")?
    };

    let pos = 8;
    let cipher_algo = u16::from_le_bytes([envelope[pos], envelope[pos + 1]]);
    let kdf_algo = u16::from_le_bytes([envelope[pos + 2], envelope[pos + 3]]);
    if cipher_algo != 1 || kdf_algo != 1 {
        return Err(Error::Message(
            "unsupported algorithm in envelope".to_string(),
        ));
    }

    let mut offset = pos + 4;
    let _time_cost = u32::from_le_bytes(envelope[offset..offset + 4].try_into().unwrap());
    let _mem_cost = u32::from_le_bytes(envelope[offset + 4..offset + 8].try_into().unwrap());
    let _parallelism = u32::from_le_bytes(envelope[offset + 8..offset + 12].try_into().unwrap());
    let salt_len =
        u16::from_le_bytes(envelope[offset + 12..offset + 14].try_into().unwrap()) as usize;
    offset += 14;
    let salt = &envelope[offset..offset + salt_len];
    offset += salt_len;

    let nonce_len = u16::from_le_bytes(envelope[offset..offset + 2].try_into().unwrap()) as usize;
    offset += 2;
    let nonce = &envelope[offset..offset + nonce_len];
    offset += nonce_len;

    let ct_len = u64::from_le_bytes(envelope[offset..offset + 8].try_into().unwrap()) as usize;
    offset += 8;
    let ciphertext = &envelope[offset..offset + ct_len];
    offset += ct_len;
    let tag_bytes = &envelope[offset..offset + 16];

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| Error::Message("decryption failed".to_string()))?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));
    let xnonce = XNonce::from_slice(nonce);

    let mut plaintext = ciphertext.to_vec();
    let tag_arr = Tag::from_slice(tag_bytes);
    let result = cipher.decrypt_in_place_detached(xnonce, &[], &mut plaintext, tag_arr);

    match result {
        Ok(_) => {
            let output_path = args
                .output
                .unwrap_or_else(|| args.data.trim_end_matches(".enc").to_string());
            atomic::write(&output_path, &plaintext)?;
            println!("decrypted: {output_path}");
            Ok(())
        }
        Err(_) => Err(Error::Message(
            "decryption failed — wrong password or corrupted data".to_string(),
        )),
    }
}
