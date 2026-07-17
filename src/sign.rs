use std::fs;

use ed25519_dalek::{Signature, Signer, SigningKey};

use crate::cli::SignArgs;
use crate::error::Error;

pub fn run(args: SignArgs) -> Result<(), Error> {
    let data = fs::read(&args.data)?;
    let secret_bytes = fs::read(&args.key)?;

    let secret_array: &[u8; 32] = secret_bytes
        .as_slice()
        .try_into()
        .map_err(|_| Error::Message("invalid secret key file (expected 32 bytes)".to_string()))?;
    let signing_key = SigningKey::from_bytes(secret_array);

    let signature: Signature = signing_key.sign(&data);

    let sig_bytes = signature.to_bytes();
    let output_path = args.output.unwrap_or_else(|| format!("{}.sig", args.data));
    fs::write(&output_path, sig_bytes)?;
    println!("signature written: {output_path}");
    Ok(())
}
