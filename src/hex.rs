use std::io::Write;

use crate::cli::HexArgs;
use crate::error::Error;

pub fn run(args: HexArgs) -> Result<(), Error> {
    let input = read_stdin_or_file(&args.data)?;
    if args.decode {
        let decoded = hex::decode(input.trim())
            .map_err(|e| Error::Message(format!("hex decode error: {e}")))?;
        std::io::stdout().write_all(&decoded).map_err(Error::Io)
    } else {
        let encoded = hex::encode(input.as_bytes());
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
