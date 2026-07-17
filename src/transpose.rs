use std::collections::BTreeSet;

use serde_json::Value;

use crate::cli::TransposeArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: TransposeArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => {
            return Err(Error::Message(
                "transpose requires an array of objects".to_string(),
            ))
        }
    };

    let mut all_keys: BTreeSet<&str> = BTreeSet::new();
    for record in records {
        if let Value::Object(obj) = record {
            for key in obj.keys() {
                all_keys.insert(key.as_str());
            }
        }
    }

    let mut transposed: Vec<Value> = Vec::new();
    let mut key_row = serde_json::Map::new();
    key_row.insert("key".to_string(), Value::String("_field".to_string()));
    for (i, key) in all_keys.iter().enumerate() {
        key_row.insert(format!("col{i}"), Value::String(key.to_string()));
    }
    transposed.push(Value::Object(key_row));

    for key in all_keys.iter() {
        let mut row = serde_json::Map::new();
        row.insert("key".to_string(), Value::String(key.to_string()));
        for (j, record) in records.iter().enumerate() {
            let val = record.get(*key).cloned().unwrap_or(Value::Null);
            row.insert(format!("col{j}"), val);
        }
        transposed.push(Value::Object(row));
    }

    let output = Value::Array(transposed);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
