use serde_json::Value;

use crate::cli::QueryArgs;
use crate::error::Error;
use crate::format;

#[derive(Debug)]
pub(crate) enum PathSegment {
    Key(String),
    Index(usize),
}

pub(crate) fn parse_path(path: &str) -> Result<Vec<PathSegment>, Error> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                if !current.is_empty() {
                    segments.push(PathSegment::Key(std::mem::take(&mut current)));
                }
            }
            '[' => {
                if !current.is_empty() {
                    segments.push(PathSegment::Key(std::mem::take(&mut current)));
                }
                let mut idx_str = String::new();
                loop {
                    match chars.next() {
                        Some(']') => break,
                        Some(c) => idx_str.push(c),
                        None => {
                            return Err(Error::Message(format!(
                                "invalid path: unmatched '[' in '{path}'"
                            )));
                        }
                    }
                }
                let idx: usize = idx_str.parse().map_err(|_| {
                    Error::Message(format!("invalid array index '{idx_str}' in path '{path}'"))
                })?;
                segments.push(PathSegment::Index(idx));
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        segments.push(PathSegment::Key(std::mem::take(&mut current)));
    }

    if segments.is_empty() {
        return Err(Error::Message(format!("empty path '{path}'")));
    }

    Ok(segments)
}

fn evaluate<'a>(value: &'a Value, segments: &[PathSegment]) -> Result<&'a Value, Error> {
    let mut current = value;
    for segment in segments {
        match segment {
            PathSegment::Key(key) => match current {
                Value::Object(map) => {
                    current = map.get(key.as_str()).ok_or_else(|| {
                        Error::Message(format!("key '{key}' not found in object"))
                    })?;
                }
                _ => {
                    return Err(Error::Message(format!(
                        "cannot access key '{key}' on non-object"
                    )));
                }
            },
            PathSegment::Index(idx) => match current {
                Value::Array(arr) => {
                    current = arr.get(*idx).ok_or_else(|| {
                        Error::Message(format!("index {idx} out of bounds (length {})", arr.len()))
                    })?;
                }
                _ => {
                    return Err(Error::Message(format!(
                        "cannot access index {idx} on non-array"
                    )));
                }
            },
        }
    }
    Ok(current)
}

pub fn run(args: QueryArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let segments = parse_path(&args.path)?;
    let result = evaluate(&value, &segments)?;

    serde_json::to_writer_pretty(std::io::stdout(), result)?;
    println!();

    Ok(())
}
