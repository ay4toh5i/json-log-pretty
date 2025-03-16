use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("json-log-pretty").unwrap();

    let input =
        "{ \"level\": \"ERROR\", \"message\": \"hello world\", \"timestamp\": \"1742106971\" }";

    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout("[15:36:11] ERROR : hello world\n");
}
