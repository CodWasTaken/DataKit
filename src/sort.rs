use serde_json::Value;

use crate::cli::SortArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: SortArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let mut records = match value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("sort requires an array".to_string())),
    };

    if args.desc {
        records.sort_by(|a, b| cmp_by_field(b, a, &args.by));
    } else {
        records.sort_by(|a, b| cmp_by_field(a, b, &args.by));
    }

    let output = Value::Array(records);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}

fn cmp_by_field(a: &Value, b: &Value, field: &str) -> std::cmp::Ordering {
    let va = a.get(field).and_then(|v| v.as_f64());
    let vb = b.get(field).and_then(|v| v.as_f64());
    match (va, vb) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        _ => {
            let sa = a.get(field).and_then(|v| v.as_str()).unwrap_or("");
            let sb = b.get(field).and_then(|v| v.as_str()).unwrap_or("");
            sa.cmp(sb)
        }
    }
}
