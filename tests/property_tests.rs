use assert_cmd::Command;

fn datakit() -> Command {
    Command::cargo_bin("datakit").expect("datakit binary not found")
}

#[test]
fn test_hex_roundtrip() {
    let inputs = vec!["hello", "", "a", "abc\x00def", "    "];
    for input in inputs {
        let encoded = datakit()
            .arg("hex")
            .arg("-")
            .write_stdin(input)
            .assert()
            .success();
        let stdout = String::from_utf8(encoded.get_output().stdout.clone()).unwrap();
        let encoded_str = stdout.trim();

        datakit()
            .arg("hex")
            .arg("-")
            .arg("--decode")
            .write_stdin(encoded_str)
            .assert()
            .success()
            .stdout(input);
    }
}

#[test]
fn test_base64_roundtrip() {
    let inputs = vec!["hello", "", "a", "abc\x00def", "     "];
    for input in inputs {
        let encoded = datakit()
            .arg("base64")
            .arg("-")
            .write_stdin(input)
            .assert()
            .success();
        let stdout = String::from_utf8(encoded.get_output().stdout.clone()).unwrap();
        let encoded_str = stdout.trim();

        datakit()
            .arg("base64")
            .arg("-")
            .arg("--decode")
            .write_stdin(encoded_str)
            .assert()
            .success()
            .stdout(input);
    }
}

#[test]
fn test_json_serialize_roundtrip() {
    let cases = vec![
        r#"{"a":1,"b":2}"#,
        r#"[1,2,3]"#,
        r#"{"nested":{"inner":"value"}}"#,
        r#"{"empty":{}}"#,
        r#"[]"#,
    ];
    for case in cases {
        let output = datakit()
            .arg("convert")
            .arg("-")
            .arg("--to")
            .arg("json")
            .write_stdin(case)
            .assert()
            .success();
        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
        let original: serde_json::Value = serde_json::from_str(case).unwrap();
        assert_eq!(parsed, original, "roundtrip failed for {case}");
    }
}

#[test]
fn test_yaml_json_roundtrip() {
    let yaml = "name: test\nvalue: 42\n";
    let output = datakit()
        .arg("convert")
        .arg("-")
        .arg("--from")
        .arg("yaml")
        .arg("--to")
        .arg("yaml")
        .write_stdin(yaml)
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("test"));
    assert!(stdout.contains("42"));
}

#[test]
fn test_toml_json_roundtrip() {
    let toml = "name = \"test\"\nvalue = 42\n";
    let output = datakit()
        .arg("convert")
        .arg("-")
        .arg("--from")
        .arg("toml")
        .arg("--to")
        .arg("toml")
        .write_stdin(toml)
        .assert()
        .success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("test"));
    assert!(stdout.contains("42"));
}
