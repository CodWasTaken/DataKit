use serde_json::Value;

use crate::cli::FilterArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: FilterArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("filter requires an array".to_string())),
    };

    let parts: Vec<&str> = args.condition.splitn(3, ' ').collect();
    if parts.len() != 3 {
        return Err(Error::Message(
            "filter condition must be 'field op value' (e.g. 'age > 30')".to_string(),
        ));
    }
    let field = parts[0];
    let op = parts[1];
    let raw_val = parts[2];

    let filtered: Vec<&Value> = records
        .iter()
        .filter(|r| match r.get(field) {
            Some(v) => match op {
                "==" => values_equal(v, raw_val),
                "!=" => !values_equal(v, raw_val),
                ">" => compare_values(v, raw_val) == Some(std::cmp::Ordering::Greater),
                "<" => compare_values(v, raw_val) == Some(std::cmp::Ordering::Less),
                ">=" => {
                    matches!(
                        compare_values(v, raw_val),
                        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
                    )
                }
                "<=" => matches!(
                    compare_values(v, raw_val),
                    Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                ),
                _ => false,
            },
            None => false,
        })
        .collect();

    let output = Value::Array(filtered.into_iter().cloned().collect());
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}

fn values_equal(v: &Value, raw: &str) -> bool {
    match v {
        Value::Number(n) => raw
            .parse::<f64>()
            .map(|f| n.as_f64() == Some(f))
            .unwrap_or(false),
        Value::String(s) => s == raw,
        Value::Bool(b) => raw.parse::<bool>().map(|x| *b == x).unwrap_or(false),
        _ => false,
    }
}

fn compare_values(v: &Value, raw: &str) -> Option<std::cmp::Ordering> {
    let a = v.as_f64()?;
    let b = raw.parse::<f64>().ok()?;
    a.partial_cmp(&b)
}
