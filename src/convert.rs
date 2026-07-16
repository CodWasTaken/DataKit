use std::fs;
use std::io::{self, Read};

use serde_json::Value;

use crate::cli::ConvertArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ConvertArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.input);
    let output_fmt = args
        .output
        .as_deref()
        .map(format::detect_format)
        .unwrap_or(format::Format::Json);

    match (input_fmt, output_fmt) {
        (format::Format::Jsonl, format::Format::Jsonl) => {
            let records = read_jsonl(&args.input)?;
            write_jsonl_stdout_or_file(&records, args.output.as_deref())?;
        }
        (format::Format::Jsonl, format::Format::Json) => {
            let records = read_jsonl(&args.input)?;
            let value = Value::Array(records);
            write_json_stdout_or_file(&value, args.output.as_deref())?;
        }
        (format::Format::Json, format::Format::Jsonl) => {
            let input = read_all(&args.input)?;
            let value: Value = serde_json::from_str(&input)?;
            let records = match value {
                Value::Array(arr) => arr,
                _ => {
                    return Err(Error::Message(
                        "converting JSON to JSONL requires a top-level array".to_string(),
                    ));
                }
            };
            write_jsonl_stdout_or_file(&records, args.output.as_deref())?;
        }
        (format::Format::Json, format::Format::Json) => {
            let input = read_all(&args.input)?;
            let value: Value = serde_json::from_str(&input)?;
            write_json_stdout_or_file(&value, args.output.as_deref())?;
        }
    }

    Ok(())
}

fn read_all(path: &str) -> Result<String, Error> {
    if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}

fn read_jsonl(path: &str) -> Result<Vec<Value>, Error> {
    if path == "-" {
        format::jsonl::read(io::stdin())
    } else {
        let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
        format::jsonl::read(file)
    }
}

fn write_jsonl_stdout_or_file(values: &[Value], output: Option<&str>) -> Result<(), Error> {
    match output {
        Some(path) => {
            let file = fs::File::create(path)?;
            format::jsonl::write(file, values)
        }
        None => {
            let mut buf: Vec<u8> = Vec::new();
            format::jsonl::write(&mut buf, values)?;
            let text =
                String::from_utf8(buf).map_err(|e| Error::Message(format!("UTF-8 error: {e}")))?;
            print!("{text}");
            Ok(())
        }
    }
}

fn write_json_stdout_or_file(value: &Value, output: Option<&str>) -> Result<(), Error> {
    match output {
        Some(path) => {
            let file = fs::File::create(path)?;
            serde_json::to_writer_pretty(file, value)?;
        }
        None => {
            let mut buf: Vec<u8> = Vec::new();
            serde_json::to_writer_pretty(&mut buf, value)?;
            let text =
                String::from_utf8(buf).map_err(|e| Error::Message(format!("UTF-8 error: {e}")))?;
            println!("{text}");
        }
    }
    Ok(())
}
