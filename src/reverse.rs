use serde_json::Value;

use crate::cli::ReverseArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ReverseArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let mut records = match value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("reverse requires an array".to_string())),
    };

    records.reverse();
    let output = Value::Array(records);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
