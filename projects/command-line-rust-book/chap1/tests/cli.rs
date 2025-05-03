use std::process::Command;

#[test]
fn should_return_0() {
    let mut cmd = Command::new("ls");
    let result = cmd.output();
    assert!(result.is_ok());
}
