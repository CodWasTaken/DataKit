use std::fs;
use std::io::{self, Read, Write};

use serde_json::Value;

use crate::cli::ConvertArgs;
use crate::error::Error;

pub fn run(args: ConvertArgs) -> Result<(), Error> {
    let input = read_input(&args.input)?;
    let value: Value = serde_json::from_str(&input)?;

    match args.output {
        Some(ref path) => {
            let file = fs::File::create(path)?;
            write_json(file, &value)?;
        }
        None => {
            let mut buf: Vec<u8> = Vec::new();
            write_json(&mut buf, &value)?;
            let text =
                String::from_utf8(buf).map_err(|e| Error::Message(format!("UTF-8 error: {e}")))?;
            println!("{text}");
        }
    }

    Ok(())
}

fn read_input(path: &str) -> Result<String, Error> {
    if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}

fn write_json<W: Write>(writer: W, value: &Value) -> Result<(), Error> {
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}
