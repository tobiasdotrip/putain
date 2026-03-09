use crate::rules::{Correction, Rule};
use crate::shell::CommandContext;
use regex::Regex;
use std::sync::LazyLock;

static RE_GIT_UPSTREAM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"git push --set-upstream (\S+) (\S+)").unwrap());
static RE_GIT_PATHSPEC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"error: pathspec '([^']+)' did not match").unwrap());
static RE_CMD_NOT_FOUND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"command not found: (\S+)").unwrap());

pub struct SudoRule;

impl Rule for SudoRule {
    fn name(&self) -> &str { "sudo" }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        let patterns = ["Permission denied", "EACCES", "permission denied", "not permitted"];
        if patterns.iter().any(|p| ctx.output.contains(p)) && !ctx.command.starts_with("sudo ") {
            Some(Correction {
                fixed_command: format!("sudo {}", ctx.command),
                rule_name: self.name().to_string(),
                confidence: 0.9,
            })
        } else {
            None
        }
    }
}

pub struct GitPushUpstreamRule;

impl Rule for GitPushUpstreamRule {
    fn name(&self) -> &str { "git_push_upstream" }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        if !ctx.command.starts_with("git push") {
            return None;
        }
        if let Some(caps) = RE_GIT_UPSTREAM.captures(&ctx.output) {
            let remote = caps.get(1)?.as_str();
            let branch = caps.get(2)?.as_str();
            Some(Correction {
                fixed_command: format!("git push --set-upstream {} {}", remote, branch),
                rule_name: self.name().to_string(),
                confidence: 0.95,
            })
        } else if ctx.output.contains("no upstream branch") {
            let branch = std::process::Command::new("git")
                .args(["branch", "--show-current"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|| "main".to_string());
            Some(Correction {
                fixed_command: format!("git push --set-upstream origin {}", branch),
                rule_name: self.name().to_string(),
                confidence: 0.8,
            })
        } else {
            None
        }
    }
}

pub struct GitCheckoutNewBranchRule;

impl Rule for GitCheckoutNewBranchRule {
    fn name(&self) -> &str { "git_checkout_new_branch" }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        if !ctx.command.starts_with("git checkout") {
            return None;
        }
        if let Some(caps) = RE_GIT_PATHSPEC.captures(&ctx.output) {
            let branch = caps.get(1)?.as_str();
            return Some(Correction {
                fixed_command: format!("git checkout -b {}", branch),
                rule_name: self.name().to_string(),
                confidence: 0.7,
            });
        }
        None
    }
}

pub struct TypoCommandRule;

impl Rule for TypoCommandRule {
    fn name(&self) -> &str { "typo_command" }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        let caps = RE_CMD_NOT_FOUND.captures(&ctx.output)?;
        let typo = caps.get(1)?.as_str();

        let known_commands = [
            "git", "docker", "cargo", "npm", "pnpm", "yarn", "node", "python", "python3",
            "pip", "pip3", "ruby", "go", "rustc", "rustup", "make", "cmake", "gcc", "g++",
            "curl", "wget", "ssh", "scp", "rsync", "ls", "cd", "mv", "cp", "rm", "cat",
            "grep", "find", "sed", "awk", "vim", "nano", "code", "brew", "apt", "dnf",
        ];

        let mut best_match: Option<(String, f64)> = None;
        for cmd in &known_commands {
            let dist = strsim::normalized_damerau_levenshtein(typo, cmd);
            if dist > 0.6 {
                if best_match.is_none() || dist > best_match.as_ref().unwrap().1 {
                    best_match = Some((cmd.to_string(), dist));
                }
            }
        }

        if let Ok(path_var) = std::env::var("PATH") {
            for dir in path_var.split(':') {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        let dist = strsim::normalized_damerau_levenshtein(typo, &name);
                        if dist > 0.7 {
                            if best_match.is_none() || dist > best_match.as_ref().unwrap().1 {
                                best_match = Some((name.into_owned(), dist));
                            }
                        }
                    }
                }
            }
        }

        let (suggestion, confidence) = best_match?;
        let first_token = ctx.command.split_whitespace().next().unwrap_or("");
        let fixed = if first_token == typo {
            let rest = ctx.command.strip_prefix(typo).unwrap_or("");
            format!("{}{}", suggestion, rest)
        } else {
            ctx.command.replacen(typo, &suggestion, 1)
        };
        Some(Correction {
            fixed_command: fixed,
            rule_name: self.name().to_string(),
            confidence,
        })
    }
}

pub struct CdTypoRule;

impl Rule for CdTypoRule {
    fn name(&self) -> &str { "cd_typo" }

    fn suggest(&self, ctx: &CommandContext) -> Option<Correction> {
        if !ctx.command.starts_with("cd ") {
            return None;
        }
        if !ctx.output.contains("no such file or directory") && !ctx.output.contains("No such file") {
            return None;
        }
        let target = ctx.command.strip_prefix("cd ")?.trim();
        let parent = std::path::Path::new(target).parent().unwrap_or(std::path::Path::new("."));
        let dir_to_scan = if parent == std::path::Path::new("") { std::path::Path::new(".") } else { parent };
        let filename = std::path::Path::new(target).file_name()?.to_string_lossy();

        let entries = std::fs::read_dir(dir_to_scan).ok()?;
        let mut best: Option<(String, f64)> = None;
        for entry in entries.flatten() {
            if entry.file_type().ok()?.is_dir() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let dist = strsim::normalized_damerau_levenshtein(&filename, &name);
                if dist > 0.5 && (best.is_none() || dist > best.as_ref().unwrap().1) {
                    let full = if parent == std::path::Path::new("") || parent == std::path::Path::new(".") {
                        name.to_string()
                    } else {
                        format!("{}/{}", parent.display(), name)
                    };
                    best = Some((full, dist));
                }
            }
        }

        let (suggestion, confidence) = best?;
        Some(Correction {
            fixed_command: format!("cd {}", suggestion),
            rule_name: self.name().to_string(),
            confidence,
        })
    }
}
