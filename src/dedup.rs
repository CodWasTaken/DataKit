use std::collections::BTreeSet;

use serde_json::Value;

use crate::cli::DedupArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: DedupArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("dedup requires an array".to_string())),
    };

    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();

    for record in records {
        let key = match &args.field {
            Some(f) => record.get(f).map(value_to_string).unwrap_or_default(),
            None => serde_json::to_string(record).unwrap_or_default(),
        };
        if seen.insert(key) {
            deduped.push(record.clone());
        }
    }

    let output = Value::Array(deduped);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}
