use std::io::{Read, Write};

use serde_json::Value;

use crate::error::Error;

pub fn read<R: Read>(reader: R) -> Result<Value, Error> {
    let mut text = String::new();
    std::io::BufReader::new(reader).read_to_string(&mut text)?;
    let value: Value =
        toml::from_str(&text).map_err(|e| Error::Message(format!("TOML error: {e}")))?;
    Ok(value)
}

pub fn write<W: Write>(writer: W, value: &Value) -> Result<(), Error> {
    match value {
        Value::Object(_) => {
            let text = toml::to_string(value)
                .map_err(|e| Error::Message(format!("TOML serialization error: {e}")))?;
            let mut buf = std::io::BufWriter::new(writer);
            std::io::Write::write_all(&mut buf, text.as_bytes())?;
            buf.flush()?;
            Ok(())
        }
        _ => Err(Error::Message(
            "TOML output requires a top-level object".to_string(),
        )),
    }
}
