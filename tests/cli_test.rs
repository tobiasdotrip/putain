use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_version_flag() {
    cargo_bin_cmd!("putain")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("putain"));
}

#[test]
fn test_help_flag() {
    cargo_bin_cmd!("putain")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("correcteur"));
}

#[test]
fn test_hook_bash_output() {
    cargo_bin_cmd!("putain")
        .args(["--hook", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("PUTAIN_CMD"));
}

#[test]
fn test_hook_zsh_output() {
    cargo_bin_cmd!("putain")
        .args(["--hook", "zsh"])
        .assert()
        .success()
        .stdout(predicates::str::contains("precmd"));
}
