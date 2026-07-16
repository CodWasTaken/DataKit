use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

use crate::cli::InspectArgs;
use crate::error::Error;

pub fn run(args: InspectArgs) -> Result<(), Error> {
    let source: Box<dyn Read> = match args.path {
        Some(ref p) => Box::new(File::open(p).map_err(|_| Error::FileNotFound(p.into()))?),
        None => Box::new(std::io::stdin()),
    };
    let value: Value = serde_json::from_reader(source)?;
    let info = describe_value(&value, 0);
    if !info.is_empty() {
        println!("{info}");
    }
    Ok(())
}

fn describe_value(v: &Value, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "empty array".to_string()
            } else {
                let mut elem_types: Vec<String> = Vec::new();
                let mut seen = BTreeSet::new();
                for elem in arr {
                    let desc = describe_value(elem, depth + 1);
                    if seen.insert(desc.clone()) {
                        elem_types.push(desc);
                    }
                }
                if elem_types.len() == 1 {
                    format!("array<{}>", elem_types[0])
                } else {
                    format!("array<{}>", elem_types.join(" | "))
                }
            }
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                "empty object".to_string()
            } else {
                let fields: BTreeMap<&String, &Value> = obj.iter().collect();
                let mut lines: Vec<String> = Vec::new();
                lines.push("{".to_string());
                for (k, v) in &fields {
                    let desc = describe_value(v, depth + 1);
                    lines.push(format!("{indent}{k}: {desc}"));
                }
                lines.push(format!("{indent}}}"));
                lines.join("\n")
            }
        }
    }
}
