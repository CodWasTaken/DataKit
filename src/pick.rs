use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde_json::Value;

use crate::cli::PickArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: PickArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("pick requires an array".to_string())),
    };

    if records.is_empty() {
        return Err(Error::Message("cannot pick from empty array".to_string()));
    }

    let seed = args.seed.unwrap_or(42);
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let picked = records.choose(&mut rng).unwrap();

    serde_json::to_writer_pretty(std::io::stdout(), picked)?;
    println!();
    Ok(())
}
