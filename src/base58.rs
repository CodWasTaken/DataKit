use std::io::Write;

use crate::cli::Base58Args;
use crate::error::Error;

pub fn run(args: Base58Args) -> Result<(), Error> {
    let input = read_stdin_or_file(&args.data)?;
    if args.decode {
        let decoded = bs58::decode(input.trim())
            .into_vec()
            .map_err(|e| Error::Message(format!("base58 decode error: {e}")))?;
        std::io::stdout().write_all(&decoded).map_err(Error::Io)
    } else {
        let encoded = bs58::encode(input.as_bytes()).into_string();
        println!("{encoded}");
        Ok(())
    }
}

fn read_stdin_or_file(path: &str) -> Result<String, Error> {
    use std::io::Read;
    if path == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path).map_err(Error::Io)
    }
}
