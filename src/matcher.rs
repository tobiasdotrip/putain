use crate::rules::{all_builtin_rules, Correction, Rule};
use crate::plugins;
use crate::config::Config;
use crate::shell::CommandContext;

pub struct Matcher {
    rules: Vec<Box<dyn Rule>>,
}

impl Matcher {
    pub fn with_builtins() -> Self {
        Self {
            rules: all_builtin_rules(),
        }
    }

    pub fn new(config: &Config) -> Self {
        let mut rules = all_builtin_rules();
        for dir in config.plugin_dirs_with_defaults() {
            rules.extend(plugins::load_plugins_from_dir(&dir));
        }
        Self { rules }
    }

    pub fn find_correction(&self, ctx: &CommandContext) -> Option<Correction> {
        let mut best: Option<Correction> = None;
        for rule in &self.rules {
            if let Some(correction) = rule.suggest(ctx) {
                if best.is_none() || correction.confidence > best.as_ref().unwrap().confidence {
                    best = Some(correction);
                }
            }
        }
        best
    }
}
