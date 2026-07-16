use serde_json::Value;

use crate::cli::HeadArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: HeadArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("head requires an array".to_string())),
    };

    let n = args.count.unwrap_or(10).min(records.len());
    let output = Value::Array(records[..n].to_vec());
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
