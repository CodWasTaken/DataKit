use serde_json::Value;

use crate::cli::FlattenArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: FlattenArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let separator = args.sep.as_deref().unwrap_or(".");

    let result = flatten_value(&value, separator);
    serde_json::to_writer_pretty(std::io::stdout(), &result)?;
    println!();
    Ok(())
}

fn flatten_value(value: &Value, sep: &str) -> Value {
    match value {
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (k, v) in obj {
                flatten_into(k, v, sep, &mut result);
            }
            Value::Object(result)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(|v| flatten_value(v, sep)).collect()),
        other => other.clone(),
    }
}

fn flatten_into(
    prefix: &str,
    value: &Value,
    sep: &str,
    result: &mut serde_json::Map<String, Value>,
) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let new_key = format!("{prefix}{sep}{k}");
                flatten_into(&new_key, v, sep, result);
            }
        }
        _ => {
            result.insert(prefix.to_string(), value.clone());
        }
    }
}
