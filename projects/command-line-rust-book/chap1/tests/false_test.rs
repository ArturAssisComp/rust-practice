use assert_cmd::Command;

const FALSE_COMMAND: &str = "false";

#[test]
fn should_return_non_zero() {
    let mut cmd = Command::cargo_bin(FALSE_COMMAND).unwrap();
    cmd.assert().failure();
}
