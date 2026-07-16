use serde_json::Value;

use crate::cli::StatsArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: StatsArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let records = match &value {
        Value::Array(arr) => arr,
        _ => {
            return Err(Error::Message(
                "stats requires an array of objects".to_string(),
            ));
        }
    };

    if records.is_empty() {
        println!("no records");
        return Ok(());
    }

    let mut numeric: Vec<(String, Vec<f64>)> = Vec::new();
    let mut string_fields: Vec<String> = Vec::new();

    if let Some(Value::Object(obj)) = records.first() {
        for key in obj.keys() {
            if records
                .iter()
                .all(|r| matches!(r.get(key), Some(Value::Number(_))))
            {
                let vals: Vec<f64> = records
                    .iter()
                    .filter_map(|r| r.get(key))
                    .filter_map(|v| v.as_f64())
                    .collect();
                numeric.push((key.clone(), vals));
            } else {
                string_fields.push(key.clone());
            }
        }
    }

    for (name, vals) in &numeric {
        let n = vals.len() as f64;
        let sum: f64 = vals.iter().sum();
        let mean = sum / n;
        let mut sorted = vals.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (min, max) = match (sorted.first(), sorted.last()) {
            (Some(&a), Some(&b)) => (a, b),
            _ => continue,
        };
        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };
        let variance: f64 = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let stddev = variance.sqrt();

        println!("{name}:");
        println!("  count: {n}");
        println!("  min:   {min}");
        println!("  max:   {max}");
        println!("  mean:  {mean:.4}");
        println!("  median: {median:.4}");
        println!("  stddev: {stddev:.4}");
    }

    for name in &string_fields {
        println!("{name}: (non-numeric)");
    }

    Ok(())
}
