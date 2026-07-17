use std::io::Write;

use crate::cli::Base32Args;
use crate::error::Error;

pub fn run(args: Base32Args) -> Result<(), Error> {
    let input = read_stdin_or_file(&args.data)?;
    if args.decode {
        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, input.trim())
            .ok_or_else(|| Error::Message("base32 decode error: invalid input".to_string()))?;
        std::io::stdout().write_all(&decoded).map_err(Error::Io)
    } else {
        let encoded = base32::encode(
            base32::Alphabet::Rfc4648 { padding: true },
            input.as_bytes(),
        );
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
