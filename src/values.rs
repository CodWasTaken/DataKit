use serde_json::Value;

use crate::cli::ValuesArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ValuesArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let vals = match &value {
        Value::Object(obj) => obj.values().cloned().collect(),
        _ => return Err(Error::Message("values requires an object".to_string())),
    };

    let output = Value::Array(vals);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
