use serde_json::Value;

use crate::cli::ExplodeArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ExplodeArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("explode requires an array".to_string())),
    };

    let mut exploded = Vec::new();
    for record in records {
        let obj = match record {
            Value::Object(m) => m,
            _ => {
                exploded.push(record.clone());
                continue;
            }
        };
        if let Some(Value::Array(items)) = obj.get(&args.field) {
            if items.is_empty() {
                let mut new_obj = obj.clone();
                new_obj.remove(&args.field);
                exploded.push(Value::Object(new_obj));
            } else {
                for item in items {
                    let mut new_obj = obj.clone();
                    new_obj.insert(args.field.clone(), item.clone());
                    exploded.push(Value::Object(new_obj));
                }
            }
        } else {
            exploded.push(record.clone());
        }
    }

    let output = Value::Array(exploded);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
