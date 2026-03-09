#[derive(Debug, Clone, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn from_env_str(s: &str) -> Option<Self> {
        let s = s.rsplit('/').next().unwrap_or(s);
        match s {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            _ => None,
        }
    }

    pub fn detect() -> Option<Self> {
        std::env::var("SHELL")
            .ok()
            .and_then(|s| Self::from_env_str(&s))
    }

    pub fn binary_name(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub command: String,
    pub output: String,
    pub exit_code: i32,
    pub shell: Shell,
}

impl CommandContext {
    /// Build context from environment variables set by the shell hook.
    /// PUTAIN_CMD, PUTAIN_OUTPUT, PUTAIN_EXIT_CODE
    pub fn from_env() -> Option<Self> {
        let command = std::env::var("PUTAIN_CMD").ok()?;
        let output = std::env::var("PUTAIN_OUTPUT").unwrap_or_default();
        let exit_code: i32 = std::env::var("PUTAIN_EXIT_CODE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let shell = Shell::detect().unwrap_or(Shell::Bash);
        Some(CommandContext {
            command,
            output,
            exit_code,
            shell,
        })
    }

    /// Build context from shell history (without re-executing the command).
    /// Fallback when env vars are not set (no hook mode).
    /// Note: output will be empty since we don't re-run the command.
    pub fn from_history() -> Option<Self> {
        let shell = Shell::detect()?;
        let last_cmd = get_last_history_command(&shell)?;
        Some(CommandContext {
            command: last_cmd,
            output: String::new(),
            exit_code: 1,
            shell,
        })
    }
}

fn get_last_history_command(shell: &Shell) -> Option<String> {
    let cmd = match shell {
        Shell::Bash => "fc -ln -1",
        Shell::Zsh => "fc -ln -1",
        Shell::Fish => "builtin history | head -1",
    };
    let output = std::process::Command::new(shell.binary_name())
        .arg("-c")
        .arg(cmd)
        .output()
        .ok()?;
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() || result == "putain" {
        None
    } else {
        Some(result)
    }
}
