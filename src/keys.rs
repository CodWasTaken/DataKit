use serde_json::Value;

use crate::cli::KeysArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: KeysArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let keys = match &value {
        Value::Object(obj) => {
            let mut ks: Vec<&String> = obj.keys().collect();
            ks.sort();
            ks.into_iter().map(|k| Value::String(k.clone())).collect()
        }
        _ => return Err(Error::Message("keys requires an object".to_string())),
    };

    let output = Value::Array(keys);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
