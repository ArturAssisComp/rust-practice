use assert_cmd::Command;
const CLI_NAME: &str = "chap1";

#[test]
fn should_run_successfully() {
    let mut cmd = Command::cargo_bin(CLI_NAME).unwrap();
    cmd.assert().success().stdout("Hello, World!\n");
}
