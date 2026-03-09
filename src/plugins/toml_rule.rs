use crate::rules::{Correction, Rule};
use crate::shell::CommandContext;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TomlRuleSet {
    #[serde(rename = "rule")]
    pub rules: Vec<TomlRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TomlRule {
    pub name: String,
    /// Optional: only match if the command starts with this
    pub command: Option<String>,
    /// Regex pattern to match against the command output
    pub output_pattern: Option<String>,
    /// The fix template. Supports {1}, {2}... for regex captures.
    /// Supports {command}, {current_branch}, {last_arg}.
    pub fix: String,
}

impl std::str::FromStr for TomlRuleSet {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

impl Rule for TomlRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        if let Some(ref cmd) = self.command
            && !ctx.command.starts_with(cmd.as_str())
        {
            return None;
        }

        let captures = if let Some(ref pattern) = self.output_pattern {
            let re = Regex::new(pattern).ok()?;
            let caps = re.captures(&ctx.output)?;
            Some(caps)
        } else {
            None
        };

        let mut fixed = self.fix.clone();

        if let Some(ref caps) = captures {
            for i in 1..caps.len() {
                if let Some(m) = caps.get(i) {
                    fixed = fixed.replace(&format!("{{{}}}", i), m.as_str());
                }
            }
        }

        fixed = fixed.replace("{command}", &ctx.command);
        fixed = fixed.replace(
            "{last_arg}",
            ctx.command.split_whitespace().last().unwrap_or(""),
        );

        if fixed.contains("{current_branch}") {
            let branch = std::process::Command::new("git")
                .args(["branch", "--show-current"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|| "main".to_string());
            fixed = fixed.replace("{current_branch}", &branch);
        }

        Some(Correction {
            fixed_command: fixed,
            rule_name: self.name.clone(),
            confidence: 0.7,
        })
    }
}
