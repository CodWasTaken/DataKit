use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde_json::Value;

use crate::cli::ShuffleArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ShuffleArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let mut records = match value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("shuffle requires an array".to_string())),
    };

    let seed = args.seed.unwrap_or(42);
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    records.shuffle(&mut rng);

    let output = Value::Array(records);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
