pub mod csv;
pub mod jsonl;
pub mod toml;
pub mod yaml;

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Jsonl,
    Csv,
    Toml,
    Yaml,
}

pub fn detect_format(path: &str) -> Format {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
        Some("jsonl" | "jsonlines" | "ndjson") => Format::Jsonl,
        Some("csv") => Format::Csv,
        Some("toml") => Format::Toml,
        Some("yaml" | "yml") => Format::Yaml,
        _ => Format::Json,
    }
}
