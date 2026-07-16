use md5::{Digest, Md5};

use crate::cli::HashArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: HashArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let json = serde_json::to_string(&value)
        .map_err(|e| Error::Message(format!("serialization error: {e}")))?;

    let hash = format!("{:x}", Md5::digest(json.as_bytes()));

    match args.algorithm.as_deref().unwrap_or("md5") {
        "md5" => println!("{hash}"),
        other => return Err(Error::Message(format!("unknown hash algorithm '{other}'"))),
    }

    Ok(())
}
