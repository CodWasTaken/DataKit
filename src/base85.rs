use std::io::Write;

use crate::cli::Base85Args;
use crate::error::Error;

pub fn run(args: Base85Args) -> Result<(), Error> {
    let input = read_stdin_or_file(&args.data)?;
    if args.decode {
        let decoded = base85::decode(input.trim())
            .ok_or_else(|| Error::Message("base85 decode error: invalid input".to_string()))?;
        std::io::stdout().write_all(&decoded).map_err(Error::Io)
    } else {
        let encoded = base85::encode(input.as_bytes());
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
