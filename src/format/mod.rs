pub mod csv;
pub mod jsonl;
pub mod toml;

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Jsonl,
    Csv,
    Toml,
}

pub fn detect_format(path: &str) -> Format {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
        Some("jsonl" | "jsonlines" | "ndjson") => Format::Jsonl,
        Some("csv") => Format::Csv,
        Some("toml") => Format::Toml,
        _ => Format::Json,
    }
}
