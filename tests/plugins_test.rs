use putain::plugins::toml_rule::TomlRuleSet;
use putain::rules::Rule;
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
fn test_parse_toml_plugin() {
    let toml_str = r#"
[[rule]]
name = "test_rule"
command = "git push"
output_pattern = "no upstream"
fix = "git push --set-upstream origin main"
"#;
    let ruleset = toml_str.parse::<TomlRuleSet>().unwrap();
    assert_eq!(ruleset.rules.len(), 1);
}

#[test]
fn test_toml_rule_matching() {
    let toml_str = r#"
[[rule]]
name = "npm_typo"
command = "npm"
output_pattern = "Did you mean (.+)\\?"
fix = "npm {1}"
"#;
    let ruleset = toml_str.parse::<TomlRuleSet>().unwrap();
    let ctx = make_ctx("npm isntall", "Did you mean install?");
    let correction = ruleset.rules[0].suggest(&ctx);
    assert!(correction.is_some());
    assert_eq!(correction.unwrap().fixed_command, "npm install");
}

#[test]
fn test_toml_rule_no_match() {
    let toml_str = r#"
[[rule]]
name = "test_rule"
command = "git push"
output_pattern = "no upstream"
fix = "git push --set-upstream origin main"
"#;
    let ruleset = toml_str.parse::<TomlRuleSet>().unwrap();
    let ctx = make_ctx("ls", "file1 file2");
    assert!(ruleset.rules[0].suggest(&ctx).is_none());
}
