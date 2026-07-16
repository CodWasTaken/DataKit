use serde_json::Value;

use crate::cli::SearchArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: SearchArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;

    let query = args.query.to_lowercase();
    let results = search_value(&value, &query, &args.field);

    for r in &results {
        println!("{r}");
    }

    Ok(())
}

fn search_value(value: &Value, query: &str, field: &Option<String>) -> Vec<String> {
    match value {
        Value::Array(arr) => {
            let mut out = Vec::new();
            for item in arr {
                if let Some(f) = field {
                    if let Some(v) = item.get(f) {
                        if let Some(s) = v.as_str() {
                            if s.to_lowercase().contains(query) {
                                out.push(s.to_string());
                            }
                        }
                    }
                } else if let Some(s) = item.as_str() {
                    if s.to_lowercase().contains(query) {
                        out.push(s.to_string());
                    }
                }
            }
            out
        }
        Value::Object(obj) => {
            let mut out = Vec::new();
            for v in obj.values() {
                if let Some(s) = v.as_str() {
                    if s.to_lowercase().contains(query) {
                        out.push(s.to_string());
                    }
                }
            }
            out
        }
        Value::String(s) => {
            if s.to_lowercase().contains(query) {
                vec![s.clone()]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}
