use std::collections::BTreeSet;
use std::io::{Read, Write};

use serde_json::Value;

use crate::error::Error;

pub fn read<R: Read>(reader: R) -> Result<Vec<Value>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let headers: Vec<String> = rdr
        .headers()
        .map_err(|e| Error::Message(format!("CSV header error: {e}")))?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut records = Vec::new();
    for (i, result) in rdr.records().enumerate() {
        let record = result.map_err(|e| Error::Message(format!("CSV row {}: {e}", i + 1)))?;
        let mut map = serde_json::Map::new();
        for (j, field) in record.iter().enumerate() {
            let key = headers.get(j).cloned().unwrap_or_else(|| format!("_{j}"));
            let value = infer_value(field);
            map.insert(key, value);
        }
        records.push(Value::Object(map));
    }
    Ok(records)
}

pub fn write<W: Write>(writer: W, values: &[Value]) -> Result<(), Error> {
    let mut wtr = csv::Writer::from_writer(writer);

    let headers = collect_headers(values);
    if headers.is_empty() {
        return Ok(());
    }

    wtr.write_record(&headers)
        .map_err(|e| Error::Message(format!("CSV write header error: {e}")))?;

    for (i, value) in values.iter().enumerate() {
        let obj = match value {
            Value::Object(map) => map,
            _ => {
                return Err(Error::Message(format!(
                    "CSV row {}: expected object, got non-object record",
                    i + 1
                )));
            }
        };
        let row: Vec<String> = headers
            .iter()
            .map(|h| obj.get(h).map(value_to_csv_string).unwrap_or_default())
            .collect();
        wtr.write_record(&row)
            .map_err(|e| Error::Message(format!("CSV write row {}: {e}", i + 1)))?;
    }

    wtr.flush()
        .map_err(|e| Error::Message(format!("CSV flush error: {e}")))?;
    Ok(())
}

fn collect_headers(values: &[Value]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    for value in values {
        if let Value::Object(map) = value {
            for key in map.keys() {
                seen.insert(key.clone());
            }
        }
    }
    seen.into_iter().collect()
}

fn infer_value(field: &str) -> Value {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        return Value::Null;
    }
    if let Ok(n) = trimmed.parse::<i64>() {
        return Value::Number(serde_json::Number::from(n));
    }
    if let Ok(n) = trimmed.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(n) {
            return Value::Number(n);
        }
    }
    if trimmed.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    if trimmed.eq_ignore_ascii_case("null") {
        return Value::Null;
    }
    Value::String(field.to_string())
}

fn value_to_csv_string(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_csv_string).collect();
            items.join(";")
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{k}:{}", value_to_csv_string(v)))
                .collect();
            items.join(",")
        }
    }
}
