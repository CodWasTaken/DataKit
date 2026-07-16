use std::io::Write;
use std::path::Path;

use crate::error::Error;

pub fn write(path: &str, data: &[u8]) -> Result<(), Error> {
    let p = Path::new(path);
    let parent = p.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| Error::Message(format!("failed to create temp file: {e}")))?;
    tmp.write_all(data)
        .map_err(|e| Error::Message(format!("failed to write temp file: {e}")))?;
    tmp.persist(p)
        .map_err(|e| Error::Message(format!("failed to rename temp file: {e}")))?;
    Ok(())
}

pub fn write_output(bytes: Vec<u8>, path: Option<&str>) -> Result<(), Error> {
    match path {
        Some(p) => write(p, &bytes),
        None => {
            let text = String::from_utf8(bytes)
                .map_err(|e| Error::Message(format!("UTF-8 error: {e}")))?;
            print!("{text}");
            Ok(())
        }
    }
}
