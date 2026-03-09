use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub plugin_dirs: Vec<PathBuf>,
    pub require_confirmation: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugin_dirs: Vec::new(),
            require_confirmation: true,
        }
    }
}

impl Config {
    pub fn plugin_dirs_with_defaults(&self) -> Vec<PathBuf> {
        let mut dirs = self.plugin_dirs.clone();
        if let Ok(exe) = std::env::current_exe()
            && let Some(parent) = exe.parent() {
                let builtin = parent.join("plugins");
                if builtin.exists() {
                    dirs.push(builtin);
                }
            }
        if let Some(config_dir) = dirs::config_dir() {
            let user_plugins = config_dir.join("putain").join("plugins");
            if user_plugins.exists() {
                dirs.push(user_plugins);
            }
        }
        dirs
    }
}
