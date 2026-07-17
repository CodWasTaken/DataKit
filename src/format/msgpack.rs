use std::io::{Read, Write};

use serde_json::Value;

use crate::error::Error;

pub fn read<R: Read>(reader: R) -> Result<Value, Error> {
    let mut buf = Vec::new();
    std::io::BufReader::new(reader).read_to_end(&mut buf)?;
    rmp_serde::from_slice::<Value>(&buf)
        .map_err(|e| Error::Message(format!("MessagePack error: {e}")))
}

pub fn write<W: Write>(mut writer: W, value: &Value) -> Result<(), Error> {
    rmp_serde::encode::write(&mut writer, value)
        .map_err(|e| Error::Message(format!("MessagePack write error: {e}")))
}
