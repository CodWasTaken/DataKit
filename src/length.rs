use serde_json::Value;

use crate::cli::LengthArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: LengthArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let n = match &value {
        Value::Array(arr) => arr.len(),
        Value::String(s) => s.len(),
        Value::Object(obj) => obj.len(),
        _ => 1,
    };

    println!("{n}");
    Ok(())
}
