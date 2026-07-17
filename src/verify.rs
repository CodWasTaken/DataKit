use std::fs;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::cli::VerifyArgs;
use crate::error::Error;

pub fn run(args: VerifyArgs) -> Result<(), Error> {
    let data = fs::read(&args.data)?;
    let sig_bytes = fs::read(&args.signature)?;
    let pub_bytes = fs::read(&args.key)?;

    if sig_bytes.len() != 64 {
        return Err(Error::Message(
            "invalid signature file (expected 64 bytes)".to_string(),
        ));
    }
    if pub_bytes.len() != 32 {
        return Err(Error::Message(
            "invalid public key file (expected 32 bytes)".to_string(),
        ));
    }

    let sig_array: &[u8; 64] = sig_bytes.as_slice().try_into().unwrap();
    let pub_array: &[u8; 32] = pub_bytes.as_slice().try_into().unwrap();

    let signature = Signature::from_bytes(sig_array);
    let public_key = VerifyingKey::from_bytes(pub_array)
        .map_err(|_| Error::Message("invalid public key".to_string()))?;

    match public_key.verify(&data, &signature) {
        Ok(_) => {
            println!("valid signature");
            Ok(())
        }
        Err(_) => {
            println!("invalid signature");
            std::process::exit(1);
        }
    }
}
