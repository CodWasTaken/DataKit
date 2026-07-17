use std::io::{BufReader, Read, Write};

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader as XmlReader;
use quick_xml::Writer as XmlWriter;
use serde_json::Value;

use crate::error::Error;

pub fn read<R: Read>(reader: R) -> Result<Value, Error> {
    let mut reader = XmlReader::from_reader(BufReader::new(reader));
    let mut buf = Vec::new();

    let mut stack: Vec<(String, serde_json::Map<String, Value>)> = Vec::new();
    let mut current_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut map = serde_json::Map::new();
                for attr in e.attributes().flatten() {
                    let k = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let v = String::from_utf8_lossy(&attr.value).to_string();
                    map.insert(format!("@{k}"), Value::String(v));
                }
                stack.push((name, map));
                current_text.clear();
            }
            Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut map = serde_json::Map::new();
                for attr in e.attributes().flatten() {
                    let k = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let v = String::from_utf8_lossy(&attr.value).to_string();
                    map.insert(format!("@{k}"), Value::String(v));
                }
                let value = if map.is_empty() {
                    Value::Null
                } else {
                    Value::Object(map)
                };
                if let Some((_, parent)) = stack.last_mut() {
                    let entry = parent
                        .entry(name)
                        .or_insert_with(|| Value::Array(Vec::new()));
                    if let Value::Array(arr) = entry {
                        arr.push(value);
                    }
                } else {
                    return Ok(value);
                }
            }
            Ok(Event::Text(ref e)) => {
                current_text = e.unescape().unwrap_or_default().to_string();
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if let Some((_, map)) = stack.pop() {
                    let value = if map.is_empty() && !current_text.trim().is_empty() {
                        Value::String(current_text.trim().to_string())
                    } else {
                        let mut m = map;
                        if !current_text.trim().is_empty() {
                            m.insert(
                                "#text".to_string(),
                                Value::String(current_text.trim().to_string()),
                            );
                        }
                        Value::Object(m)
                    };
                    if let Some((_, parent)) = stack.last_mut() {
                        let entry = parent
                            .entry(name)
                            .or_insert_with(|| Value::Array(Vec::new()));
                        if let Value::Array(arr) = entry {
                            arr.push(value);
                        }
                    } else {
                        return Ok(value);
                    }
                    current_text.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::Message(format!("XML error: {e}"))),
            _ => {}
        }
        buf.clear();
    }

    Err(Error::Message("empty XML document".to_string()))
}

pub fn write<W: Write>(writer: W, value: &Value) -> Result<(), Error> {
    let mut writer = XmlWriter::new_with_indent(writer, b' ', 2);

    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))
        .map_err(|e| Error::Message(format!("XML write error: {e}")))?;

    match value {
        Value::Object(obj) => {
            write_object(&mut writer, obj)?;
        }
        _ => {
            return Err(Error::Message(
                "XML output requires a root object".to_string(),
            ))
        }
    }

    writer.into_inner().flush()?;
    Ok(())
}

fn write_object<W: Write>(
    writer: &mut XmlWriter<W>,
    obj: &serde_json::Map<String, Value>,
) -> Result<(), Error> {
    for (name, value) in obj {
        if name.starts_with('@') || name == "#text" {
            continue;
        }
        match value {
            Value::Object(child) => {
                writer
                    .write_event(Event::Start(BytesStart::new(name.as_str())))
                    .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
                write_object(writer, child)?;
                writer
                    .write_event(Event::End(BytesEnd::new(name.as_str())))
                    .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
            }
            Value::Array(arr) => {
                for item in arr {
                    writer
                        .write_event(Event::Start(BytesStart::new(name.as_str())))
                        .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
                    write_value(writer, item)?;
                    writer
                        .write_event(Event::End(BytesEnd::new(name.as_str())))
                        .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
                }
            }
            _ => {
                writer
                    .write_event(Event::Start(BytesStart::new(name.as_str())))
                    .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
                write_value(writer, value)?;
                writer
                    .write_event(Event::End(BytesEnd::new(name.as_str())))
                    .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
            }
        }
    }
    Ok(())
}

fn write_value<W: Write>(writer: &mut XmlWriter<W>, value: &Value) -> Result<(), Error> {
    match value {
        Value::String(s) => {
            writer
                .write_event(Event::Text(BytesText::new(s)))
                .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
        }
        Value::Number(n) => {
            writer
                .write_event(Event::Text(BytesText::new(&n.to_string())))
                .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
        }
        Value::Bool(b) => {
            writer
                .write_event(Event::Text(BytesText::new(if *b {
                    "true"
                } else {
                    "false"
                })))
                .map_err(|e| Error::Message(format!("XML write error: {e}")))?;
        }
        Value::Object(obj) => {
            write_object(writer, obj)?;
        }
        Value::Array(arr) => {
            for item in arr {
                write_value(writer, item)?;
            }
        }
        Value::Null => {}
    }
    Ok(())
}
