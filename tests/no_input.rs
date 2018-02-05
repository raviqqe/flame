use std::process::Command;

#[test]
fn no_input() {
    let output = Command::new("./target/debug/flame")
        .arg("-h")
        .output()
        .unwrap();

    assert!(String::from_utf8_lossy(&output.stderr).contains("Flame programming language"));
}
