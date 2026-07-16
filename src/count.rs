use crate::cli::CountArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: CountArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let count = match &value {
        serde_json::Value::Array(arr) => arr.len(),
        serde_json::Value::Object(_) => 1,
        _ => 1,
    };

    println!("{count}");
    Ok(())
}
