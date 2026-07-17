use std::io::Write;

use base64::Engine;

use crate::cli::Base64Args;
use crate::error::Error;

pub fn run(args: Base64Args) -> Result<(), Error> {
    let input = read_stdin_or_file(&args.data)?;
    if args.decode {
        let engine = if args.url {
            base64::engine::general_purpose::URL_SAFE
        } else {
            base64::engine::general_purpose::STANDARD
        };
        let decoded = engine
            .decode(input.trim())
            .map_err(|e| Error::Message(format!("base64 decode error: {e}")))?;
        std::io::stdout().write_all(&decoded).map_err(Error::Io)
    } else {
        let engine = if args.url {
            base64::engine::general_purpose::URL_SAFE
        } else {
            base64::engine::general_purpose::STANDARD
        };
        let encoded = engine.encode(input.as_bytes());
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
