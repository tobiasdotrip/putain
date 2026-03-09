use putain::rules::Rule;
use putain::rules::builtin;
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
fn test_sudo_rule() {
    let rule = builtin::SudoRule;
    let ctx = make_ctx("apt install foo", "Permission denied");
    let correction = rule.suggest(&ctx);
    assert!(correction.is_some());
    assert_eq!(correction.unwrap().fixed_command, "sudo apt install foo");
}

#[test]
fn test_sudo_rule_no_match() {
    let rule = builtin::SudoRule;
    let ctx = make_ctx("ls", "file1 file2");
    assert!(rule.suggest(&ctx).is_none());
}

#[test]
fn test_git_push_upstream_rule() {
    let rule = builtin::GitPushUpstreamRule;
    let ctx = make_ctx(
        "git push",
        "fatal: The current branch feat has no upstream branch.\nTo push the current branch and set the remote as upstream, use\n\n    git push --set-upstream origin feat\n",
    );
    let correction = rule.suggest(&ctx);
    assert!(correction.is_some());
    assert_eq!(
        correction.unwrap().fixed_command,
        "git push --set-upstream origin feat"
    );
}

#[test]
fn test_typo_command_rule() {
    let rule = builtin::TypoCommandRule;
    let ctx = make_ctx("gti status", "zsh: command not found: gti");
    let correction = rule.suggest(&ctx);
    assert!(correction.is_some());
    assert_eq!(correction.unwrap().fixed_command, "git status");
}
