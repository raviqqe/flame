use std::process::Command;

#[test]
fn no_input() {
    let s = String::from_utf8(
        Command::new("./target/debug/flame")
            .arg("-h")
            .output()
            .unwrap()
            .stderr,
    ).unwrap();

    assert!(s.contains("Flame programming language"));
    assert!(s.contains("Options"));
}
