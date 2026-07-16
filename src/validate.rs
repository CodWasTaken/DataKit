use std::fs;
use std::io::Read;

use serde_json::Value;

use crate::cli::ValidateArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: ValidateArgs) -> Result<(), Error> {
    let schema_text = read_file(&args.schema)?;
    let schema: Value = serde_json::from_str(&schema_text)
        .map_err(|e| Error::Message(format!("invalid JSON Schema file: {e}")))?;

    let data_fmt = format::detect_format(&args.data);
    let data = crate::convert::read_input(&args.data, data_fmt)?;

    let validator = jsonschema::validator_for(&schema)
        .map_err(|e| Error::Message(format!("invalid JSON Schema: {e}")))?;

    let mut errors: Vec<String> = Vec::new();
    for error in validator.iter_errors(&data) {
        errors.push(format!("  at {}: {}", error.instance_path(), error));
    }

    if errors.is_empty() {
        println!("valid");
    } else {
        println!(
            "invalid ({} error{})",
            errors.len(),
            if errors.len() == 1 { "" } else { "s" }
        );
        for err in &errors {
            println!("{err}");
        }
        return Err(Error::Message("validation failed".to_string()));
    }

    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    if path == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}
