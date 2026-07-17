use std::collections::BTreeMap;

use serde_json::Value;

use crate::cli::GroupArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: GroupArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("group requires an array".to_string())),
    };

    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for record in records {
        let key = record
            .get(&args.by)
            .map(value_to_string)
            .unwrap_or_default();
        groups.entry(key).or_default().push(record.clone());
    }

    let result: Vec<Value> = groups
        .into_iter()
        .map(|(k, v)| serde_json::json!({"key": k, "count": v.len(), "items": Value::Array(v)}))
        .collect();

    let output = Value::Array(result);
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
