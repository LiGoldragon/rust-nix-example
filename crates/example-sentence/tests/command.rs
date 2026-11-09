//! Runs the built command, found the way cargo names it.

use std::process::Command;

#[test]
fn command_describes_its_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_example-sentence"))
        .args(["quiet", "river"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "2 words: [{\"word\":\"quiet\"},{\"word\":\"river\"}]\n"
    );
}
