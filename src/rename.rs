use serde_json::Value;

use crate::cli::RenameArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: RenameArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let parts: Vec<&str> = args.mapping.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(Error::Message("mapping must be 'old:new'".to_string()));
    }
    let old = parts[0];
    let new = parts[1];

    let result = rename_in_value(&value, old, new);

    serde_json::to_writer_pretty(std::io::stdout(), &result)?;
    println!();
    Ok(())
}

fn rename_in_value(value: &Value, old: &str, new: &str) -> Value {
    match value {
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                let key = if k == old { new.to_string() } else { k.clone() };
                map.insert(key, rename_in_value(v, old, new));
            }
            Value::Object(map)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| rename_in_value(v, old, new)).collect())
        }
        other => other.clone(),
    }
}
