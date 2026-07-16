use serde_json::Value;

use crate::cli::MergeArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: MergeArgs) -> Result<(), Error> {
    let fmt_a = format::detect_format(&args.file_a);
    let fmt_b = format::detect_format(&args.file_b);
    let val_a = crate::convert::read_input(&args.file_a, fmt_a)?;
    let val_b = crate::convert::read_input(&args.file_b, fmt_b)?;

    let merged = match (&val_a, &val_b) {
        (Value::Object(a), Value::Object(b)) => {
            let mut map = a.clone();
            for (k, v) in b {
                map.insert(k.clone(), v.clone());
            }
            Value::Object(map)
        }
        (Value::Array(a), Value::Array(b)) => {
            let mut arr = a.clone();
            arr.extend(b.clone());
            Value::Array(arr)
        }
        _ => {
            return Err(Error::Message(
                "merge requires two objects or two arrays".to_string(),
            ))
        }
    };

    serde_json::to_writer_pretty(std::io::stdout(), &merged)?;
    println!();
    Ok(())
}
