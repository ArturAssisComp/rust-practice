use assert_cmd::Command;

const TRUE_COMMAND: &str = "true";

#[test]
fn should_return_successful() {
    let mut cmd = Command::cargo_bin(TRUE_COMMAND).unwrap();
    cmd.assert().success();
}
