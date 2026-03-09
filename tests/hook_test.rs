use putain::hook;

#[test]
fn test_bash_hook_contains_key_elements() {
    let script = hook::generate_hook(putain::shell::Shell::Bash);
    assert!(script.contains("PUTAIN_CMD"));
    assert!(script.contains("PUTAIN_OUTPUT"));
    assert!(script.contains("PUTAIN_EXIT_CODE"));
    assert!(script.contains("putain"));
}

#[test]
fn test_zsh_hook_contains_key_elements() {
    let script = hook::generate_hook(putain::shell::Shell::Zsh);
    assert!(script.contains("precmd"));
    assert!(script.contains("PUTAIN_CMD"));
}

#[test]
fn test_fish_hook_contains_key_elements() {
    let script = hook::generate_hook(putain::shell::Shell::Fish);
    assert!(script.contains("PUTAIN_CMD"));
    assert!(script.contains("fish_postexec"));
}
