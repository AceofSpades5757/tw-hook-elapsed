//! WARNING: When returning a task to a string, it will change the order of the attributes.
//!
//! Available Binaries:
//~ * on-add_elapsed.rs
//~ * on-add_noop.rs
//~ * on-modify_elapsed.rs
//~ * on-modify_noop.rs
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

/// Test a binary to see if the processed input becomes the expected output.
fn test_io(binary: &str, input: &str, expected: &str) {
    let mut command = Command::new("cargo")
        .args(["run", "--bin", binary])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("binary runs successfully");

    if let Some(stdin) = command.stdin.as_mut() {
        stdin
            .write_all(input.as_bytes())
            .expect("write bytes to stdin");
    }
    let output = command
        .wait_with_output()
        .expect("receive output from completed process");

    assert!(output.status.success());
    let output_string: &str = &String::from_utf8_lossy(&output.stdout);
    assert_eq!(output_string, expected);
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn test() {
    let input = r#"{"description":"test","entry":"20210101T000000Z","modified":"20210101T000000Z","status":"pending","tags":["test"],"uuid":"00000000000000000000000000000000",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z"
    }"#.replace('\n', "");
    let expected = r#"{"uuid":"00000000-0000-0000-0000-000000000000","description":"test",
        "start":"20210101T020000Z",
        "end":"20210101T040000Z",
        "entry":"20210101T000000Z","modified":"20210101T000000Z","status":"pending","tags":["test"],"elapsed":"PT7200S"}"#
        // Remove spacing from readability
        .replace("    ", "").replace('\n', "");

    test_io("on-add_elapsed", &input, &expected);
}
