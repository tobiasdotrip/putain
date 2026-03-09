use putain::shell::CommandContext;

#[test]
fn test_command_context_from_parts() {
    let ctx = CommandContext {
        command: "git push".to_string(),
        output: "fatal: no upstream branch".to_string(),
        exit_code: 1,
        shell: putain::shell::Shell::Zsh,
    };
    assert_eq!(ctx.command, "git push");
    assert_eq!(ctx.exit_code, 1);
}

#[test]
fn test_shell_detection() {
    use putain::shell::Shell;
    assert_eq!(Shell::from_env_str("zsh"), Some(Shell::Zsh));
    assert_eq!(Shell::from_env_str("/bin/bash"), Some(Shell::Bash));
    assert_eq!(Shell::from_env_str("fish"), Some(Shell::Fish));
    assert_eq!(Shell::from_env_str("unknown"), None);
}
