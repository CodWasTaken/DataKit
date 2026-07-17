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
        .stderr(predicate::str::contains("file not found"));
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
        .stderr(predicate::str::contains("JSON error"));
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
        .stderr(predicate::str::contains("JSON error"));
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

#[test]
fn test_inspect_yaml_file() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("config.yaml");
    std::fs::write(&file, "name: MyApp\nversion: 1\nactive: true\n").unwrap();

    datakit()
        .arg("inspect")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("version: number"))
        .stdout(predicate::str::contains("active: boolean"));
}

#[test]
fn test_convert_yaml_to_json() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.yaml");
    let output = dir.path().join("data.json");
    std::fs::write(&input, "x: 10\ny: hello\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!({"x": 10, "y": "hello"}));
}

#[test]
fn test_convert_json_to_yaml() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("data.yaml");
    std::fs::write(&input, r#"{"a":1,"b":"two"}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert!(result.contains("a:"));
    assert!(result.contains("1"));
    assert!(result.contains("b:"));
    assert!(result.contains("two"));
}

#[test]
fn test_convert_yaml_to_yaml_roundtrip() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.yaml");
    let output = dir.path().join("output.yaml");
    std::fs::write(&input, "name: Bob\nscore: 88\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert!(result.contains("Bob"));
}

#[test]
fn test_convert_indent_4() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("out.json");
    std::fs::write(&input, r#"{"a":1,"b":2}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .arg("--indent")
        .arg("4")
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert_eq!(result, "{\n    \"a\": 1,\n    \"b\": 2\n}");
}

#[test]
fn test_convert_indent_0_minified() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    let output = dir.path().join("out.json");
    std::fs::write(&input, r#"{"a":1,"b":2}"#).unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .arg("--indent")
        .arg("0")
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    assert_eq!(result.trim(), r#"{"a":1,"b":2}"#);
}

#[test]
fn test_completions_bash() {
    datakit()
        .arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("_datakit"));
}

#[test]
fn test_completions_zsh() {
    datakit()
        .arg("completions")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn test_completions_fish() {
    datakit()
        .arg("completions")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_completions_help() {
    datakit()
        .arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Shell to generate"));
}

#[test]
fn test_stats_numeric() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":1},{"x":2},{"x":3},{"x":4},{"x":5}]"#).unwrap();

    datakit()
        .arg("stats")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("count: 5"))
        .stdout(predicate::str::contains("min:   1"))
        .stdout(predicate::str::contains("max:   5"))
        .stdout(predicate::str::contains("mean:  3.0000"))
        .stdout(predicate::str::contains("median: 3.0000"));
}

#[test]
fn test_filter_equals() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(
        &file,
        r#"[{"name":"Alice"},{"name":"Bob"},{"name":"Alice"}]"#,
    )
    .unwrap();

    datakit()
        .arg("filter")
        .arg(&file)
        .arg("--condition")
        .arg("name == Alice")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""name": "Alice""#));
}

#[test]
fn test_filter_greater() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":1},{"x":5},{"x":3}]"#).unwrap();

    datakit()
        .arg("filter")
        .arg(&file)
        .arg("--condition")
        .arg("x > 2")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""x": 5"#))
        .stdout(predicate::str::contains(r#""x": 3"#));
}

#[test]
fn test_filter_no_match() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":1},{"x":2}]"#).unwrap();

    datakit()
        .arg("filter")
        .arg(&file)
        .arg("--condition")
        .arg("x > 10")
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn test_select_fields() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(
        &file,
        r#"[{"a":1,"b":"x","c":true},{"a":2,"b":"y","c":false}]"#,
    )
    .unwrap();

    datakit()
        .arg("select")
        .arg(&file)
        .arg("--fields")
        .arg("a,b")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""a": 1"#))
        .stdout(predicate::str::contains(r#""b": "x""#));
}

#[test]
fn test_select_single_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"name":"Alice","age":30,"active":true}"#).unwrap();

    datakit()
        .arg("select")
        .arg(&file)
        .arg("--fields")
        .arg("name")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""name": "Alice""#));
}

#[test]
fn test_diff_identical() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.json");
    let b = dir.path().join("b.json");
    std::fs::write(&a, r#"{"x":1}"#).unwrap();
    std::fs::write(&b, r#"{"x":1}"#).unwrap();

    datakit()
        .arg("diff")
        .arg(&a)
        .arg(&b)
        .assert()
        .success()
        .stdout(predicate::str::contains("identical"));
}

#[test]
fn test_diff_different() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.json");
    let b = dir.path().join("b.json");
    std::fs::write(&a, r#"{"x":1}"#).unwrap();
    std::fs::write(&b, r#"{"x":2}"#).unwrap();

    datakit()
        .arg("diff")
        .arg(&a)
        .arg(&b)
        .assert()
        .success()
        .stdout(predicate::str::contains("-"))
        .stdout(predicate::str::contains("+"));
}

#[test]
fn test_convert_from_to_format() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.dat");
    let output = dir.path().join("out.dat");
    std::fs::write(&input, "{\"a\":1,\"b\":2}").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .arg("--from")
        .arg("json")
        .arg("--to")
        .arg("json")
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!({"a": 1, "b": 2}));
}

#[test]
fn test_convert_from_yaml_no_extension() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data");
    let output = dir.path().join("output.json");
    std::fs::write(&input, "x: 42\n").unwrap();

    datakit()
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .arg("--from")
        .arg("yaml")
        .assert()
        .success();

    let result = std::fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, serde_json::json!({"x": 42}));
}

#[test]
fn test_sort_asc() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":3},{"x":1},{"x":2}]"#).unwrap();

    datakit()
        .arg("sort")
        .arg(&file)
        .arg("--by")
        .arg("x")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""x": 1"#));
}

#[test]
fn test_sort_desc() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":1},{"x":3},{"x":2}]"#).unwrap();

    datakit()
        .arg("sort")
        .arg(&file)
        .arg("--by")
        .arg("x")
        .arg("--desc")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""x": 3"#));
}

#[test]
fn test_reverse() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a","b","c"]"#).unwrap();

    datakit().arg("reverse").arg(&file).assert().success();
}

#[test]
fn test_fill_nulls() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"name":null,"x":1}"#).unwrap();

    datakit()
        .arg("fill")
        .arg(&file)
        .arg("--field")
        .arg("name")
        .arg("--value")
        .arg("UNKNOWN")
        .assert()
        .success()
        .stdout(predicate::str::contains("UNKNOWN"));
}

#[test]
fn test_explode_array_field() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(
        &file,
        r#"[{"id":1,"items":["a","b"]},{"id":2,"items":["c"]}]"#,
    )
    .unwrap();

    datakit()
        .arg("explode")
        .arg(&file)
        .arg("--field")
        .arg("items")
        .assert()
        .success();
}

#[test]
fn test_count_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":1}"#).unwrap();

    datakit()
        .arg("count")
        .arg(&file)
        .assert()
        .success()
        .stdout("1\n");
}

#[test]
fn test_rename_field() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"old_name":"Alice"}"#).unwrap();

    datakit()
        .arg("rename")
        .arg(&file)
        .arg("--mapping")
        .arg("old_name:new_name")
        .assert()
        .success()
        .stdout(predicate::str::contains("new_name"))
        .stdout(predicate::str::contains("old_name").not());
}

#[test]
fn test_dedup_field() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"x":1},{"x":2},{"x":1}]"#).unwrap();

    datakit()
        .arg("dedup")
        .arg(&file)
        .arg("--field")
        .arg("x")
        .assert()
        .success();
}

#[test]
fn test_entries() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":1,"b":2}"#).unwrap();

    datakit()
        .arg("entries")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""key": "a""#));
}

#[test]
fn test_flatten_object() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":{"b":{"c":1}},"d":2}"#).unwrap();

    datakit()
        .arg("flatten")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("a.b.c"))
        .stdout(predicate::str::contains("d"));
}

#[test]
fn test_slice_start() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a","b","c","d"]"#).unwrap();

    datakit()
        .arg("slice")
        .arg(&file)
        .arg("--start")
        .arg("1")
        .arg("--end")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""b""#))
        .stdout(predicate::str::contains(r#""c""#));
}

#[test]
fn test_slice_out_of_range() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a"]"#).unwrap();

    datakit()
        .arg("slice")
        .arg(&file)
        .arg("--start")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn test_sample_count() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[1,2,3,4,5,6,7,8,9,10]"#).unwrap();

    datakit()
        .arg("sample")
        .arg(&file)
        .arg("--count")
        .arg("3")
        .arg("--seed")
        .arg("42")
        .assert()
        .success();
}

#[test]
fn test_shuffle_deterministic() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"v":1},{"v":2},{"v":3}]"#).unwrap();

    datakit()
        .arg("shuffle")
        .arg(&file)
        .arg("--seed")
        .arg("42")
        .assert()
        .success();
}

#[test]
fn test_head() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a","b","c","d","e"]"#).unwrap();

    datakit()
        .arg("head")
        .arg(&file)
        .arg("--count")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""a""#))
        .stdout(predicate::str::contains(r#""b""#));
}

#[test]
fn test_tail() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a","b","c","d","e"]"#).unwrap();

    datakit()
        .arg("tail")
        .arg(&file)
        .arg("--count")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""d""#))
        .stdout(predicate::str::contains(r#""e""#));
}

#[test]
fn test_stats_empty() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("empty.json");
    std::fs::write(&file, "[]").unwrap();

    datakit()
        .arg("stats")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("no records"));
}

#[test]
fn test_stats_non_numeric() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"name":"Alice"},{"name":"Bob"}]"#).unwrap();

    datakit()
        .arg("stats")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("non-numeric"));
}

#[test]
fn test_merge_objects() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.json");
    let b = dir.path().join("b.json");
    std::fs::write(&a, r#"{"x":1}"#).unwrap();
    std::fs::write(&b, r#"{"y":2}"#).unwrap();

    datakit().arg("merge").arg(&a).arg(&b).assert().success();
}

#[test]
fn test_merge_arrays() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.json");
    let b = dir.path().join("b.json");
    std::fs::write(&a, r#"["a","b"]"#).unwrap();
    std::fs::write(&b, r#"["c","d"]"#).unwrap();

    datakit().arg("merge").arg(&a).arg(&b).assert().success();
}

#[test]
fn test_pick() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"["a","b","c"]"#).unwrap();

    datakit()
        .arg("pick")
        .arg(&file)
        .arg("--seed")
        .arg("42")
        .assert()
        .success();
}

#[test]
fn test_zip() {
    let dir = TempDir::new().unwrap();
    let a = dir.path().join("a.json");
    let b = dir.path().join("b.json");
    std::fs::write(&a, r#"["a","b"]"#).unwrap();
    std::fs::write(&b, r#"[1,2]"#).unwrap();

    datakit().arg("zip").arg(&a).arg(&b).assert().success();
}

#[test]
fn test_keys() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"b":2,"a":1}"#).unwrap();
    datakit().arg("keys").arg(&file).assert().success();
}

#[test]
fn test_values() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"x":42,"y":"hello"}"#).unwrap();
    datakit().arg("values").arg(&file).assert().success();
}

#[test]
fn test_round() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":3.14159}"#).unwrap();
    datakit()
        .arg("round")
        .arg(&file)
        .arg("--decimals")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("3.14"));
}

#[test]
fn test_hash_md5() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":1}"#).unwrap();
    datakit()
        .arg("hash")
        .arg(&file)
        .arg("--algorithm")
        .arg("md5")
        .assert()
        .success();
}

#[test]
fn test_base64_encode() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.txt");
    std::fs::write(&file, b"hello").unwrap();
    datakit()
        .arg("base64")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}

#[test]
fn test_base64_decode() {
    datakit()
        .arg("base64")
        .arg("-")
        .arg("--decode")
        .write_stdin("aGVsbG8=")
        .assert()
        .success()
        .stdout("hello");
}

#[test]
fn test_hex_encode() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.txt");
    std::fs::write(&file, b"hello").unwrap();
    datakit()
        .arg("hex")
        .arg(&file)
        .assert()
        .success()
        .stdout("68656c6c6f\n");
}

#[test]
fn test_hex_decode() {
    datakit()
        .arg("hex")
        .arg("-")
        .arg("--decode")
        .write_stdin("68656c6c6f")
        .assert()
        .success()
        .stdout("hello");
}

#[test]
fn test_pretty() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"a":1,"b":2}"#).unwrap();
    datakit()
        .arg("pretty")
        .arg(&file)
        .arg("--indent")
        .arg("4")
        .assert()
        .success();
}

#[test]
fn test_check_valid() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"{"ok":true}"#).unwrap();
    datakit()
        .arg("check")
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_check_invalid() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"not json"#).unwrap();
    datakit()
        .arg("check")
        .arg(&file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

#[test]
fn test_search() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[{"name":"Alice"},{"name":"Bob"}]"#).unwrap();
    datakit()
        .arg("search")
        .arg(&file)
        .arg("--query")
        .arg("ice")
        .arg("--field")
        .arg("name")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_length_array() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("data.json");
    std::fs::write(&file, r#"[1,2,3]"#).unwrap();
    datakit()
        .arg("length")
        .arg(&file)
        .assert()
        .success()
        .stdout("3\n");
}
