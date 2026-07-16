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
