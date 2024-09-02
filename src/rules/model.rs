use pnet::packet::ethernet::EthernetPacket;
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub action: String, // allow or deny
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<String>,
}

impl Rule {
    pub fn matches(&self, packet: &EthernetPacket) -> bool {
        // Implement matching logic here, e.g., check IP, port, protocol
        true
    }

    pub fn save(new_rule: Rule) {
        config::config_files::ensure_config_dir_exists();

        let rules = config::config_files::read_config_file("rules.json");
        let mut rules: Vec<Rule> = match rules {
            Some(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
            None => vec![],
        };

        rules.push(new_rule);

        let serialized = serde_json::to_string(&rules).unwrap();
        config::config_files::write_config_file("rules.json", &serialized);
    }

    pub fn load() -> Vec<Rule> {
        config::config_files::ensure_config_dir_exists();
        let rules = config::config_files::read_config_file("rules.json");

        match rules {
            Some(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| vec![]),
            None => vec![],
        }
    }
}
