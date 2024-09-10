use crate::infrastructure::rule;
use pnet::packet::ethernet::EthernetPacket;
use serde::{Deserialize, Serialize};

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
        println!("Matching rule: {:?}", self);
        println!("Packet: {:?}", packet);

        // Example logic, replace with actual matching logic:
        if let Some(ref _ip) = self.ip {
            // Check IP
        }
        if let Some(_port) = self.port {
            // Check port
        }
        if let Some(ref _protocol) = self.protocol {
            // Check protocol
        }

        true // Placeholder
    }

    pub fn add_rule(allow: bool, deny: bool, port: u16, ip: &String, protocol: &String) {
        let action = if allow {
            "allow".to_string()
        } else if deny {
            "deny".to_string()
        } else {
            panic!("Either allow or deny must be true");
        };

        let new_rule = Self {
            action,
            ip: Some(ip.to_string()),
            port: Some(port),
            protocol: Some(protocol.to_string()),
        };

        rule::save(new_rule);
    }

    pub fn get_rules() -> Vec<Self> {
        return rule::load();
    }
}
