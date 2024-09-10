use crate::{core::rule::Rule, infrastructure::config};

pub fn save(new_rule: Rule) {
    config::ensure_config_dir_exists();

    let rules = config::read_config_file("rules.json");
    let mut rules: Vec<Rule> = match rules {
        Some(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
        None => vec![],
    };

    rules.push(new_rule);

    let serialized = serde_json::to_string(&rules).unwrap();
    config::write_config_file("rules.json", &serialized);
}

pub fn load() -> Vec<Rule> {
    config::ensure_config_dir_exists();
    let rules = config::read_config_file("rules.json");

    match rules {
        Some(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
        None => vec![],
    }
}
