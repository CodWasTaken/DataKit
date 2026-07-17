use std::fs;

use serde_json::Value;

use crate::cli::PipelineArgs;
use crate::error::Error;
use crate::format;

#[derive(serde::Deserialize)]
struct PipelineConfig {
    #[serde(default)]
    steps: Vec<Step>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct Step {
    name: Option<String>,
    read: Option<String>,
    write: Option<String>,
    filter: Option<String>,
    select: Option<Vec<String>>,
    sort: Option<String>,
    sort_desc: Option<bool>,
}

pub fn run(args: PipelineArgs) -> Result<(), Error> {
    let content = fs::read_to_string(&args.pipeline)?;
    let config: PipelineConfig = serde_yaml::from_str(&content)
        .map_err(|e| Error::Message(format!("invalid pipeline file: {e}")))?;

    if args.dry_run {
        println!("pipeline valid: {} step(s)", config.steps.len());
        for (i, step) in config.steps.iter().enumerate() {
            let name = step.name.as_deref().unwrap_or("unnamed");
            println!("  step {i}: {name}");
        }
        return Ok(());
    }

    let mut current: Option<Value> = None;

    for (i, step) in config.steps.iter().enumerate() {
        let name = step.name.as_deref().unwrap_or("unnamed");

        if let Some(ref path) = step.read {
            let fmt = format::detect_format(path);
            current = Some(crate::convert::read_input(path, fmt)?);
            continue;
        }

        let data = current.as_ref().ok_or_else(|| {
            Error::Message(format!("step {i} ({name}): no data from previous step"))
        })?;

        if let Some(ref condition) = step.filter {
            let parts: Vec<&str> = condition.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let field = parts[0];
                let op = parts[1];
                let val = parts[2];
                if let Value::Array(arr) = data {
                    let filtered: Vec<Value> = arr
                        .iter()
                        .filter(|r| match r.get(field) {
                            Some(v) => match op {
                                "==" => {
                                    v.as_str() == Some(val) || v.as_f64() == val.parse::<f64>().ok()
                                }
                                ">" => v.as_f64() > val.parse::<f64>().ok(),
                                "<" => v.as_f64() < val.parse::<f64>().ok(),
                                _ => false,
                            },
                            None => false,
                        })
                        .cloned()
                        .collect();
                    current = Some(Value::Array(filtered));
                }
            }
            continue;
        }

        if let Some(ref fields) = step.select {
            let field_set: std::collections::BTreeSet<&str> =
                fields.iter().map(|s| s.as_str()).collect();
            match data {
                Value::Array(arr) => {
                    let selected: Vec<Value> = arr
                        .iter()
                        .map(|item| match item {
                            Value::Object(obj) => {
                                let mut map = serde_json::Map::new();
                                for (k, v) in obj {
                                    if field_set.contains(k.as_str()) {
                                        map.insert(k.clone(), v.clone());
                                    }
                                }
                                Value::Object(map)
                            }
                            other => other.clone(),
                        })
                        .collect();
                    current = Some(Value::Array(selected));
                }
                Value::Object(obj) => {
                    let mut map = serde_json::Map::new();
                    for (k, v) in obj {
                        if field_set.contains(k.as_str()) {
                            map.insert(k.clone(), v.clone());
                        }
                    }
                    current = Some(Value::Object(map));
                }
                _ => {}
            }
            continue;
        }

        if let Some(ref sort_field) = step.sort {
            if let Value::Array(arr) = data {
                let mut sorted = arr.clone();
                let desc = step.sort_desc.unwrap_or(false);
                if desc {
                    sorted.sort_by(|a, b| cmp_field(b, a, sort_field));
                } else {
                    sorted.sort_by(|a, b| cmp_field(a, b, sort_field));
                }
                current = Some(Value::Array(sorted));
            }
            continue;
        }

        if let Some(ref path) = step.write {
            let fmt = format::detect_format(path);
            let content = serialize(data, fmt)?;
            crate::atomic::write(path, &content)?;
            continue;
        }
    }

    Ok(())
}

fn cmp_field(a: &Value, b: &Value, field: &str) -> std::cmp::Ordering {
    let va = a.get(field).and_then(|v| v.as_f64());
    let vb = b.get(field).and_then(|v| v.as_f64());
    match (va, vb) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        _ => {
            let sa = a.get(field).and_then(|v| v.as_str()).unwrap_or("");
            let sb = b.get(field).and_then(|v| v.as_str()).unwrap_or("");
            sa.cmp(sb)
        }
    }
}

fn serialize(value: &Value, fmt: format::Format) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    match fmt {
        format::Format::Json => serde_json::to_writer_pretty(&mut buf, value)?,
        format::Format::Yaml => format::yaml::write(&mut buf, value)?,
        format::Format::Toml => format::toml::write(&mut buf, value)?,
        format::Format::Csv => {
            if let Value::Array(arr) = value {
                format::csv::write(&mut buf, arr)?;
            }
        }
        format::Format::Jsonl => {
            if let Value::Array(arr) = value {
                format::jsonl::write(&mut buf, arr)?;
            }
        }
        format::Format::Xml => format::xml::write(&mut buf, value)?,
        format::Format::Msgpack => format::msgpack::write(&mut buf, value)?,
    }
    Ok(buf)
}
