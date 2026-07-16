use base64::Engine;

use crate::cli::EncodeArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: EncodeArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let json = serde_json::to_string(&value)
        .map_err(|e| Error::Message(format!("serialization error: {e}")))?;

    match args.algorithm.as_deref().unwrap_or("base64") {
        "base64" => {
            let encoded = base64::engine::general_purpose::STANDARD.encode(&json);
            println!("{encoded}");
        }
        "hex" => {
            let encoded = hex::encode(&json);
            println!("{encoded}");
        }
        other => return Err(Error::Message(format!("unknown encoding '{other}'"))),
    }

    Ok(())
}
