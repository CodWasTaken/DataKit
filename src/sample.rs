use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde_json::Value;

use crate::cli::SampleArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: SampleArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => return Err(Error::Message("sample requires an array".to_string())),
    };

    let count = args.count.unwrap_or(1).min(records.len());
    let seed = args.seed.unwrap_or(42);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..records.len()).collect();
    indices.shuffle(&mut rng);

    let sampled: Vec<Value> = indices[..count]
        .iter()
        .map(|&i| records[i].clone())
        .collect();
    let output = Value::Array(sampled);
    serde_json::to_writer_pretty(std::io::stdout(), &output)?;
    println!();
    Ok(())
}
