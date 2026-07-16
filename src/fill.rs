use serde_json::Value;

use crate::cli::FillArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: FillArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let result = fill_value(&value, &args.field, &args.value);

    serde_json::to_writer_pretty(std::io::stdout(), &result)?;
    println!();
    Ok(())
}

fn fill_value(value: &Value, field: &str, fill_val: &str) -> Value {
    match value {
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                if k == field && matches!(v, Value::Null) {
                    map.insert(k.clone(), Value::String(fill_val.to_string()));
                } else {
                    map.insert(k.clone(), fill_value(v, field, fill_val));
                }
            }
            Value::Object(map)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| fill_value(v, field, fill_val)).collect())
        }
        other => other.clone(),
    }
}
