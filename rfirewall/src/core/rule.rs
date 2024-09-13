use crate::infrastructure::rule;
use pnet::packet::ethernet::EthernetPacket;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub action: String, // allow or deny
    pub ip: String,
    pub port: u16,
    pub protocol: String,
}

impl Rule {
    pub fn matches(&self, packet: &EthernetPacket) -> bool {
        // Implement matching logic here, e.g., check IP, port, protocol
        println!("Matching rule: {:?}", self);
        println!("Packet: {:?}", packet);

        // Example logic, replace with actual matching logic:
        // if let ref _ip = self.ip {
        //     // Check IP
        // }
        // if let _port = self.port {
        //     // Check port
        // }
        // if let ref _protocol = self.protocol {
        //     // Check protocol
        // }

        true // Placeholder
    }

    pub fn new(allow: bool, port: u16, ip: &str, protocol: &str) -> Self {
        let action = if allow {
            "allow".to_string()
        } else {
            "deny".to_string()
        };

        let new_rule = Self {
            action,
            ip: ip.to_string(),
            port: port,
            protocol: protocol.to_string(),
        };

        rule::save(&new_rule);

        new_rule
    }

    pub fn get_rules() -> Vec<Self> {
        return rule::load();
    }
}
