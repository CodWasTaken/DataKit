use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn datakit() -> Command {
    Command::cargo_bin("datakit").expect("datakit binary not found")
}

#[test]
fn test_inspect_json_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("test.json");
    std::fs::write(&file, r#"{"name":"Alice","age":30,"active":true}"#).unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("age: number"))
        .stdout(predicate::str::contains("active: boolean"));
}

#[test]
fn test_inspect_empty_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.json");
    std::fs::write(&file, "{}").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("empty object"));
}

#[test]
fn test_inspect_empty_array() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.json");
    std::fs::write(&file, "[]").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("empty array"));
}

#[test]
fn test_inspect_nested_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("nested.json");
    std::fs::write(
        &file,
        r#"{"user":{"name":"Bob","scores":[1,2,3],"meta":{"role":"admin"}}}"#,
    )
    .unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("scores: array<number>"))
        .stdout(predicate::str::contains("role: string"));
}

#[test]
fn test_inspect_array_with_mixed_types() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("mixed.json");
    std::fs::write(&file, r#"[1,"hello",true]"#).unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("array<number | string | boolean>"));
}

#[test]
fn test_inspect_scalar_null() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("null.json");
    std::fs::write(&file, "null").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("null"));
}

#[test]
fn test_inspect_scalar_number() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("num.json");
    std::fs::write(&file, "42.5").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("number"));
}

#[test]
fn test_convert_file_to_file() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.json");
    let output = dir.path().join("output.json");
    std::fs::write(&input, r#"{"a":1,"b":2}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!({"a": 1, "b": 2}));
}

#[test]
fn test_convert_file_to_stdout() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.json");
    std::fs::write(&input, r#"{"hello":"world"}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("world"));
}

#[test]
fn test_convert_stdin_to_stdout() {
    datakit()
        .arg("convert")
        .arg("-")
        .write_stdin(r#"{"x":1,"y":2}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("x"))
        .stdout(predicate::str::contains("y"));
}

#[test]
fn test_inspect_missing_file() {
    datakit()
        .arg("inspect")
        .arg("/nonexistent/file.json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("FileNotFound"));
}

#[test]
fn test_convert_invalid_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("bad.json");
    std::fs::write(&input, "not valid json").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Json"));
}

#[test]
fn test_inspect_invalid_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("bad.json");
    std::fs::write(&input, "{invalid}").unwrap();

    datakit()
        .arg("inspect")
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Json"));
}

#[test]
fn test_inspect_jsonl_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.jsonl");
    std::fs::write(
        &file,
        r#"{"id":1,"name":"Alice"}
{"id":2,"name":"Bob"}
{"id":3,"name":"Charlie"}
"#,
    )
    .unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("jsonl records: 3"))
        .stdout(predicate::str::contains("id: number"))
        .stdout(predicate::str::contains("name: string"));
}

#[test]
fn test_inspect_jsonl_empty() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.jsonl");
    std::fs::write(&file, "").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("jsonl records: 0"));
}

#[test]
fn test_convert_jsonl_to_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.jsonl");
    let output = dir.path().join("data.json");
    std::fs::write(
        &input,
        r#"{"x":1}
{"x":2}
{"x":3}
"#,
    )
    .unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!([{"x": 1}, {"x": 2}, {"x": 3}]));
}

#[test]
fn test_convert_json_to_jsonl() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.jsonl");
    std::fs::write(&input, r#"[{"a":1},{"a":2}]"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], r#"{"a":1}"#);
    assert_eq!(lines[1], r#"{"a":2}"#);
}

#[test]
fn test_convert_jsonl_to_jsonl() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.jsonl");
    let output = dir.path().join("output.jsonl");
    std::fs::write(
        &input,
        r#"{"v":10}
{"v":20}
"#,
    )
    .unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], r#"{"v":10}"#);
    assert_eq!(lines[1], r#"{"v":20}"#);
}

#[test]
fn test_convert_json_to_jsonl_non_array() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.jsonl");
    std::fs::write(&input, r#"{"not":"an array"}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires a top-level array"));
}

#[test]
fn test_inspect_jsonl_invalid_line() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("bad.jsonl");
    std::fs::write(
        &file,
        r#"{"valid": true}
invalid json
{"also": "valid"}
"#,
    )
    .unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("JSONL line 2"));
}

#[test]
fn test_inspect_csv_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.csv");
    std::fs::write(&file, "name,age,active\nAlice,30,true\nBob,25,false\n").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("csv rows: 2"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("age: number"))
        .stdout(predicate::str::contains("active: boolean"));
}

#[test]
fn test_inspect_csv_empty() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.csv");
    std::fs::write(&file, "a,b,c\n").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("csv rows: 0"));
}

#[test]
fn test_convert_csv_to_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.csv");
    let output = dir.path().join("data.json");
    std::fs::write(&input, "x,y\n1,2\n3,4\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(
        parsed,
        serde_json::json!([{"x": 1, "y": 2}, {"x": 3, "y": 4}])
    );
}

#[test]
fn test_convert_json_to_csv() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.csv");
    std::fs::write(&input, r#"[{"a":1,"b":"x"},{"a":2,"b":"y"}]"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert!(result.contains("a,b"));
    assert!(result.contains("1,x"));
    assert!(result.contains("2,y"));
}

#[test]
fn test_convert_csv_to_jsonl() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.csv");
    let output = dir.path().join("data.jsonl");
    std::fs::write(&input, "v\n10\n20\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], r#"{"v":10}"#);
    assert_eq!(lines[1], r#"{"v":20}"#);
}

#[test]
fn test_convert_json_to_csv_non_array() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.csv");
    std::fs::write(&input, r#"42"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires a top-level array"));
}

#[test]
fn test_convert_csv_to_csv_roundtrip() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.csv");
    let output = dir.path().join("output.csv");
    std::fs::write(&input, "name,score\nAlice,95\nBob,87\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert_eq!(result, "name,score\nAlice,95\nBob,87\n");
}

#[test]
fn test_help_available() {
    datakit()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("Commands"));
}

#[test]
fn test_inspect_help() {
    datakit()
        .arg("inspect")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Inspect the structure"));
}

#[test]
fn test_convert_help() {
    datakit()
        .arg("convert")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Convert data"));
}

#[test]
fn test_validate_json_valid() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    std::fs::write(
        &schema,
        r#"{"type": "object", "properties": {"name": {"type": "string"}}, "required": ["name"]}"#,
    )
    .unwrap();
    std::fs::write(&data, r#"{"name": "Alice"}"#).unwrap();

    datakit()
        .arg("validate")
        .arg(&data)
        .arg("--schema")
        .arg(&schema)
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_validate_json_invalid() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    std::fs::write(
        &schema,
        r#"{"type": "object", "properties": {"name": {"type": "string"}}, "required": ["name"]}"#,
    )
    .unwrap();
    std::fs::write(&data, r#"{"age": 30}"#).unwrap();

    datakit()
        .arg("validate")
        .arg(&data)
        .arg("--schema")
        .arg(&schema)
        .assert()
        .failure()
        .stdout(predicate::str::contains("invalid"))
        .stdout(predicate::str::contains("name"));
}

#[test]
fn test_validate_invalid_schema_file() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    std::fs::write(&schema, r#"not valid json"#).unwrap();
    std::fs::write(&data, r#"{}"#).unwrap();

    datakit()
        .arg("validate")
        .arg(&data)
        .arg("--schema")
        .arg(&schema)
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid JSON Schema file"));
}

#[test]
fn test_validate_help() {
    datakit()
        .arg("validate")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validate data"));
}

#[test]
fn test_query_simple_field() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"name":"Alice","age":30}"#).unwrap();

    datakit()
        .arg("query")
        .arg(&file)
        .arg("--path")
        .arg("name")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_query_nested_field() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"user":{"name":"Bob","scores":[1,2,3]}}"#).unwrap();

    datakit()
        .arg("query")
        .arg(&file)
        .arg("--path")
        .arg("user.name")
        .assert()
        .success()
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn test_query_array_index() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"items":[10,20,30]}"#).unwrap();

    datakit()
        .arg("query")
        .arg(&file)
        .arg("--path")
        .arg("items[1]")
        .assert()
        .success()
        .stdout(predicate::str::contains("20"));
}

#[test]
fn test_query_nested_array_and_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"users":[{"name":"Alice"},{"name":"Bob"}]}"#).unwrap();

    datakit()
        .arg("query")
        .arg(&file)
        .arg("--path")
        .arg("users[0].name")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_query_missing_key() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":1}"#).unwrap();

    datakit()
        .arg("query")
        .arg(&file)
        .arg("--path")
        .arg("b")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_query_help() {
    datakit()
        .arg("query")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Query a field path"));
}

#[test]
fn test_inspect_toml_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("config.toml");
    std::fs::write(
        &file,
        "title = \"MyApp\"\n[server]\nhost = \"localhost\"\nport = 8080\n",
    )
    .unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("title: string"))
        .stdout(predicate::str::contains("server"))
        .stdout(predicate::str::contains("host: string"))
        .stdout(predicate::str::contains("port: number"));
}

#[test]
fn test_convert_toml_to_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("config.toml");
    let output = dir.path().join("config.json");
    std::fs::write(&input, "name = \"test\"\nvalue = 42\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!({"name": "test", "value": 42}));
}

#[test]
fn test_convert_json_to_toml() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.toml");
    std::fs::write(&input, r#"{"title":"MyApp","version":1}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert!(result.contains("title"));
    assert!(result.contains("MyApp"));
    assert!(result.contains("version"));
}

#[test]
fn test_convert_toml_to_toml_roundtrip() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.toml");
    let output = dir.path().join("output.toml");
    std::fs::write(&input, "name = \"Alice\"\nscore = 95.5\nactive = true\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert!(result.contains("Alice"));
    assert!(result.contains("95.5"));
}
