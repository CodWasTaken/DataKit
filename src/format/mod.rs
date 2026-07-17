pub mod csv;
pub mod jsonl;
pub mod msgpack;
pub mod toml;
pub mod xml;
pub mod yaml;

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Jsonl,
    Csv,
    Toml,
    Yaml,
    Xml,
    Msgpack,
}

pub fn parse_format_name(name: &str) -> Result<Format, crate::error::Error> {
    match name.to_lowercase().as_str() {
        "json" => Ok(Format::Json),
        "jsonl" | "jsonlines" | "ndjson" => Ok(Format::Jsonl),
        "csv" => Ok(Format::Csv),
        "toml" => Ok(Format::Toml),
        "yaml" | "yml" => Ok(Format::Yaml),
        "xml" => Ok(Format::Xml),
        "msgpack" | "mpk" => Ok(Format::Msgpack),
        _ => Err(crate::error::Error::Message(format!(
            "unknown format '{name}' (supported: json, jsonl, csv, toml, yaml, xml, msgpack)"
        ))),
    }
}

pub fn detect_format(path: &str) -> Format {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
        Some("jsonl" | "jsonlines" | "ndjson") => Format::Jsonl,
        Some("csv") => Format::Csv,
        Some("toml") => Format::Toml,
        Some("yaml" | "yml") => Format::Yaml,
        Some("xml") => Format::Xml,
        Some("msgpack" | "mpk") => Format::Msgpack,
        _ => Format::Json,
    }
}
