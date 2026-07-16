use serde_json::Value;

use crate::cli::SliceArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: SliceArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("slice requires an array".to_string())),
    };

    let start: usize = args.start.unwrap_or(0);
    let end = args.end.unwrap_or(records.len()).min(records.len());

    if start > end || start >= records.len() {
        let output = Value::Array(Vec::new());
        serde_json::to_writer_pretty(std::io::stdout(), &output)?;
        println!();
        return Ok(());
    }

    let sliced: Vec<Value> = records[start..end].to_vec();
    let output = Value::Array(sliced);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
