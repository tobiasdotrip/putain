use std::collections::HashMap;

pub struct Personality {
    counters: HashMap<String, u32>,
}

impl Personality {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn react(&mut self, rule_name: &str) -> String {
        let count = self.counters.entry(rule_name.to_string()).or_insert(0);
        *count += 1;
        let level = *count;

        match level {
            1 => pick_level1(),
            2 => pick_level2(),
            3 => pick_level3(),
            _ => pick_level4(),
        }
    }
}

fn pick_level1() -> String {
    let messages = [
        "Putain...",
        "Oh putain.",
        "Putain, allez...",
        "Merde, putain.",
        "Roh putain...",
        "Ah putain.",
    ];
    messages[random_index(messages.len())].to_string()
}

fn pick_level2() -> String {
    let messages = [
        "Oh putain, encore ?!",
        "Putain mais encore !",
        "Sérieux, encore putain ?",
        "Encore ?! Putain...",
        "Non mais putain, encore ?",
    ];
    messages[random_index(messages.len())].to_string()
}

fn pick_level3() -> String {
    let messages = [
        "PUTAIN MAIS C'EST PAS POSSIBLE",
        "PUTAIN DE BORDEL DE MERDE",
        "OH PUTAIN MAIS APPRENDS À TAPER",
        "PUTAIN TROIS FOIS LA MÊME ERREUR",
        "MAIS PUTAIN C'EST PAS VRAI",
    ];
    messages[random_index(messages.len())].to_string()
}

fn pick_level4() -> String {
    let messages = [
        "... (soupir) ...",
        "...... putain ......",
        "... (désespoir silencieux) ...",
        "... bon. encore. d'accord. ...",
        "... je dis plus rien ...",
    ];
    messages[random_index(messages.len())].to_string()
}

fn random_index(len: usize) -> usize {
    // Simple randomness without pulling in rand crate
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    t % len
}
