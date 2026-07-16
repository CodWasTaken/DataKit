use std::collections::BTreeSet;

use serde_json::Value;

use crate::cli::SelectArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: SelectArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let fields: Vec<&str> = args.fields.split(',').map(|s| s.trim()).collect();

    match value {
        Value::Array(arr) => {
            let selected: Vec<Value> = arr
                .iter()
                .map(|item| select_fields(item, &fields))
                .collect();
            let output = Value::Array(selected);
            serde_json::to_writer_pretty(std::io::stdout(), &output)?;
            println!();
        }
        Value::Object(obj) => {
            let selected = select_fields(&Value::Object(obj), &fields);
            serde_json::to_writer_pretty(std::io::stdout(), &selected)?;
            println!();
        }
        _ => {
            return Err(Error::Message(
                "select requires an object or array of objects".to_string(),
            ));
        }
    }

    Ok(())
}

fn select_fields(value: &Value, fields: &[&str]) -> Value {
    match value {
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            let field_set: BTreeSet<&str> = fields.iter().copied().collect();
            for (k, v) in obj {
                if field_set.contains(k.as_str()) {
                    map.insert(k.clone(), v.clone());
                }
            }
            Value::Object(map)
        }
        other => other.clone(),
    }
}
