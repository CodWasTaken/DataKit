use sha2::{Digest, Sha256, Sha512};

use crate::cli::HashArgs;
use crate::error::Error;

pub fn run(args: HashArgs) -> Result<(), Error> {
    let algorithm = args.algorithm.as_deref().unwrap_or("sha256");
    let input = args.data.as_bytes();

    let hash = match algorithm {
        "md5" => {
            eprintln!(
                "warning: MD5 is insecure and should not be used for security-sensitive workflows"
            );
            format!("{:x}", md5::Md5::digest(input))
        }
        "sha256" => format!("{:x}", Sha256::digest(input)),
        "sha512" => format!("{:x}", Sha512::digest(input)),
        other => return Err(Error::Message(format!("unknown hash algorithm '{other}'"))),
    };

    println!("{hash}");
    Ok(())
}
