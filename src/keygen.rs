use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use ed25519_dalek::SigningKey;
use rand::RngCore;

use crate::cli::KeygenArgs;
use crate::error::Error;

pub fn run(args: KeygenArgs) -> Result<(), Error> {
    let mut secret_bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    let secret_path = &args.output;
    let public_path = format!("{}.pub", secret_path);

    #[cfg(unix)]
    {
        fs::write(secret_path, signing_key.to_bytes())?;
        let mut perms = fs::metadata(secret_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(secret_path, perms)?;
    }

    #[cfg(not(unix))]
    fs::write(secret_path, signing_key.to_bytes())?;

    let pub_bytes = verifying_key.to_bytes();
    fs::write(&public_path, pub_bytes)?;

    let fingerprint = blake3::hash(&pub_bytes).to_string();
    println!("generated key pair:");
    println!("  secret: {secret_path}");
    println!("  public: {public_path}");
    println!("  fingerprint: {fingerprint}");

    Ok(())
}
