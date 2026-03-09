pub mod toml_rule;

use crate::rules::Rule;
use std::path::Path;

pub fn load_plugins_from_dir(dir: &Path) -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return rules,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "toml") {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    if let Ok(ruleset) = toml_rule::TomlRuleSet::from_str(&content) {
                        for rule in ruleset.rules {
                            rules.push(Box::new(rule));
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    }
    rules
}
