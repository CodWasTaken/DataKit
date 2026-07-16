use serde_json::Value;

use crate::cli::EntriesArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: EntriesArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let obj = match &value {
        Value::Object(o) => o,
        _ => return Err(Error::Message("entries requires an object".to_string())),
    };

    let entries: Vec<Value> = obj
        .iter()
        .map(|(k, v)| serde_json::json!({"key": k, "value": v}))
        .collect();

    let output = Value::Array(entries);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
