use std::fs;
use std::io::{self, Read};

use serde::Serialize;
use serde_json::Value;

use crate::atomic;

use crate::cli::ConvertArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ConvertArgs) -> Result<(), Error> {
    let input_fmt = match &args.from {
        Some(f) => format::parse_format_name(f)?,
        None => format::detect_format(&args.input),
    };
    let output_fmt = match &args.to {
        Some(f) => format::parse_format_name(f)?,
        None => args
            .output
            .as_deref()
            .map(format::detect_format)
            .unwrap_or(format::Format::Json),
    };
    let indent = args.indent;

    let value = read_input(&args.input, input_fmt)?;
    write_output(&value, args.output.as_deref(), output_fmt, indent)?;

    Ok(())
}

pub(crate) fn read_input(path: &str, fmt: format::Format) -> Result<Value, Error> {
    match fmt {
        format::Format::Json => {
            let text = read_all(path)?;
            Ok(serde_json::from_str(&text)?)
        }
        format::Format::Jsonl => {
            let records = if path == "-" {
                format::jsonl::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::jsonl::read(file)?
            };
            Ok(Value::Array(records))
        }
        format::Format::Csv => {
            let records = if path == "-" {
                format::csv::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::csv::read(file)?
            };
            Ok(Value::Array(records))
        }
        format::Format::Toml => {
            let value = if path == "-" {
                format::toml::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::toml::read(file)?
            };
            Ok(value)
        }
        format::Format::Yaml => {
            let value = if path == "-" {
                format::yaml::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::yaml::read(file)?
            };
            Ok(value)
        }
        format::Format::Xml => {
            let value = if path == "-" {
                format::xml::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::xml::read(file)?
            };
            Ok(value)
        }
        format::Format::Msgpack => {
            let value = if path == "-" {
                format::msgpack::read(io::stdin())?
            } else {
                let file = fs::File::open(path).map_err(|_| Error::FileNotFound(path.into()))?;
                format::msgpack::read(file)?
            };
            Ok(value)
        }
    }
}

fn write_output(
    value: &Value,
    path: Option<&str>,
    fmt: format::Format,
    indent: usize,
) -> Result<(), Error> {
    match fmt {
        format::Format::Json => {
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                if indent == 0 {
                    serde_json::to_writer(&mut buf, value)?;
                } else if indent == 2 {
                    serde_json::to_writer_pretty(&mut buf, value)?;
                } else {
                    use serde_json::ser::PrettyFormatter;
                    use serde_json::ser::Serializer;
                    let indent_bytes = vec![b' '; indent];
                    let fmt = PrettyFormatter::with_indent(&indent_bytes);
                    let mut ser = Serializer::with_formatter(&mut buf, fmt);
                    value.serialize(&mut ser)?;
                }
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Jsonl => {
            let records = match value {
                Value::Array(arr) => arr.clone(),
                _ => {
                    return Err(Error::Message(
                        "converting to JSONL requires a top-level array".to_string(),
                    ));
                }
            };
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::jsonl::write(&mut buf, &records)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Csv => {
            let records = match value {
                Value::Array(arr) => arr.clone(),
                _ => {
                    return Err(Error::Message(
                        "converting to CSV requires a top-level array".to_string(),
                    ));
                }
            };
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::csv::write(&mut buf, &records)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Toml => {
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::toml::write(&mut buf, value)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Yaml => {
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::yaml::write(&mut buf, value)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Xml => {
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::xml::write(&mut buf, value)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
        format::Format::Msgpack => {
            let content = || -> Result<Vec<u8>, Error> {
                let mut buf = Vec::new();
                format::msgpack::write(&mut buf, value)?;
                Ok(buf)
            };
            write_bytes(content()?, path)
        }
    }
}

fn write_bytes(bytes: Vec<u8>, path: Option<&str>) -> Result<(), Error> {
    atomic::write_output(bytes, path)
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
