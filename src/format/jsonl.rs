use std::io::{self, BufRead, BufReader, Read, Write};

use serde_json::Value;

use crate::error::Error;

pub fn read<R: Read>(reader: R) -> Result<Vec<Value>, Error> {
    let buf = BufReader::new(reader);
    let mut records = Vec::new();
    for (i, line) in buf.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let value: Value = serde_json::from_str(trimmed)
            .map_err(|e| Error::Message(format!("JSONL line {}: {e}", i + 1)))?;
        records.push(value);
    }
    Ok(records)
}

pub fn write<W: Write>(writer: W, values: &[Value]) -> Result<(), Error> {
    let mut buf = io::BufWriter::new(writer);
    for value in values {
        serde_json::to_writer(&mut buf, value)?;
        buf.write_all(b"\n")?;
    }
    buf.flush()?;
    Ok(())
}
