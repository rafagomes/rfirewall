use std::process;

use pnet::{
    datalink::{self, Channel::Ethernet},
    packet::ethernet::EthernetPacket,
};

use crate::rules;

pub fn start_monitor() {
    let interfaces = datalink::interfaces();
    let rules = rules::get_rules::get_rules();

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No interfaces were found");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            eprintln!(
                "An error occurred when creating the datalink channel: {}",
                e
            );
            process::exit(1);
        }
    };

    println!("Monitoring on interface: {}", interface.name);

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet = EthernetPacket::new(packet).unwrap();
                handle_packet(&ethernet, &rules);
            }
            Err(e) => {
                eprintln!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(packet: &EthernetPacket, rules: &Vec<rules::model::Rule>) {
    println!("Received packet: {:?}", packet);

    for rule in rules {
        if rule.matches(packet) {
            match rule.action.as_str() {
                "allow" => println!("Packet allowed: {:?}", packet),
                "deny" => println!("Packet denied: {:?}", packet),
                _ => (),
            }
            return;
        }
    }
}
