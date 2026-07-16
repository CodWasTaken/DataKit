use std::collections::BTreeSet;

use serde_json::Value;

use crate::cli::UniqueArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: UniqueArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("unique requires an array".to_string())),
    };

    let mut seen = BTreeSet::new();
    for record in records {
        if let Some(v) = record.get(&args.field) {
            let s = value_to_string(v);
            seen.insert(s);
        }
    }

    for val in &seen {
        println!("{val}");
    }

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
