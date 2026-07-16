use serde_json::Value;

use crate::cli::ZipArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ZipArgs) -> Result<(), Error> {
    let fmt_a = format::detect_format(&args.file_a);
    let fmt_b = format::detect_format(&args.file_b);
    let val_a = crate::convert::read_input(&args.file_a, fmt_a)?;
    let val_b = crate::convert::read_input(&args.file_b, fmt_b)?;

    let arr_a = match &val_a {
        Value::Array(a) => a,
        _ => return Err(Error::Message("zip requires two arrays".to_string())),
    };
    let arr_b = match &val_b {
        Value::Array(b) => b,
        _ => return Err(Error::Message("zip requires two arrays".to_string())),
    };

    let zipped: Vec<Value> = arr_a
        .iter()
        .zip(arr_b.iter())
        .map(|(a, b)| serde_json::json!([a, b]))
        .collect();

    let output = Value::Array(zipped);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
