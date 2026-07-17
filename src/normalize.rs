use serde_json::Value;

use crate::cli::NormalizeArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: NormalizeArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("normalize requires an array".to_string())),
    };

    let mut out = Vec::new();
    for record in records {
        normalize_record(record, &args.field, &mut out);
    }

    let output = Value::Array(out);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}

fn normalize_record(record: &Value, field: &str, out: &mut Vec<Value>) {
    match record {
        Value::Object(obj) => {
            if let Some(Value::Array(arr)) = obj.get(field) {
                for item in arr {
                    let mut new_obj = obj.clone();
                    new_obj.insert(field.to_string(), item.clone());
                    normalize_record(&Value::Object(new_obj), field, out);
                }
            } else {
                out.push(record.clone());
            }
        }
        _ => out.push(record.clone()),
    }
}
