use assert_cmd::Command;

#[test]
fn test_version_flag() {
    Command::cargo_bin("putain")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("putain"));
}

#[test]
fn test_help_flag() {
    Command::cargo_bin("putain")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("correcteur"));
}

#[test]
fn test_hook_bash_output() {
    Command::cargo_bin("putain")
        .unwrap()
        .args(["--hook", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("PUTAIN_CMD"));
}

#[test]
fn test_hook_zsh_output() {
    Command::cargo_bin("putain")
        .unwrap()
        .args(["--hook", "zsh"])
        .assert()
        .success()
        .stdout(predicates::str::contains("precmd"));
}
