use putain::matcher::Matcher;
use putain::shell::{CommandContext, Shell};

fn make_ctx(command: &str, output: &str) -> CommandContext {
    CommandContext {
        command: command.to_string(),
        output: output.to_string(),
        exit_code: 1,
        shell: Shell::Zsh,
    }
}

#[test]
fn test_matcher_finds_best_correction() {
    let matcher = Matcher::with_builtins();
    let ctx = make_ctx("apt install foo", "Permission denied");
    let correction = matcher.find_correction(&ctx);
    assert!(correction.is_some());
    assert_eq!(correction.unwrap().fixed_command, "sudo apt install foo");
}

#[test]
fn test_matcher_no_match() {
    let matcher = Matcher::with_builtins();
    let ctx = make_ctx("echo hello", "hello");
    let correction = matcher.find_correction(&ctx);
    assert!(correction.is_none());
}
