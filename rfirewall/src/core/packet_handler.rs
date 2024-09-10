use crate::core::rule;
use pnet::packet::ethernet::EthernetPacket;

pub fn handle_packet(packet: &EthernetPacket, rules_option: &Option<Vec<rule::Rule>>) {
    println!("Received packet: {:?}", packet);

    if let Some(rules) = rules_option {
        for rule in rules {
            if rule.matches(packet) {
                match rule.action.as_str() {
                    "allow" => println!("Packet allowed: {:?}", packet),
                    "deny" => println!("Packet denied: {:?}", packet),
                    _ => println!("Default Packet allowed: {:?}", packet),
                }
                return;
            }
        }
    } else {
        println!(
            "No rules defined, default action: Packet allowed: {:?}",
            packet
        );
    }
}
