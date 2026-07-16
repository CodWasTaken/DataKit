use serde_json::Value;

use crate::cli::RoundArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: RoundArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let decimals = args.decimals.unwrap_or(0);
    let factor = 10f64.powi(decimals as i32);

    let result = round_value(&value, factor);
    serde_json::to_writer_pretty(std::io::stdout(), &result)?;
    println!();
    Ok(())
}

fn round_value(value: &Value, factor: f64) -> Value {
    match value {
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                let rounded = (f * factor).round() / factor;
                if rounded.fract() == 0.0 && rounded.abs() < i64::MAX as f64 {
                    Value::Number(serde_json::Number::from(rounded as i64))
                } else {
                    serde_json::Number::from_f64(rounded)
                        .map(Value::Number)
                        .unwrap_or(Value::Number(n.clone()))
                }
            } else {
                value.clone()
            }
        }
        Value::Array(arr) => Value::Array(arr.iter().map(|v| round_value(v, factor)).collect()),
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                map.insert(k.clone(), round_value(v, factor));
            }
            Value::Object(map)
        }
        other => other.clone(),
    }
}
