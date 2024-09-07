use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use pnet::{
    datalink::{self, Channel::Ethernet},
    packet::ethernet::EthernetPacket,
};

use crate::rules;

pub fn start_monitor(running: Arc<AtomicBool>) {
    let interfaces = datalink::interfaces();
    let rules = rules::get_rules::get_rules();

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No interfaces were found");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            eprintln!("Unhandled channel type");
            process::exit(1); // Exit the program
        }
        Err(e) => {
            eprintln!(
                "An error occurred when creating the datalink channel: {}",
                e
            );
            process::exit(1);
        }
    };

    println!("Monitoring on interface: {}", interface.name);

    let rules_option = if rules.is_empty() { None } else { Some(rules) };

    while running.load(Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                let ethernet = EthernetPacket::new(packet).unwrap();
                handle_packet(&ethernet, &rules_option);
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::Interrupted {
                    eprintln!("An error occurred while reading packets: {}", e);
                } else {
                    break;
                }
            }
        }
    }
}

pub fn stop_monitor(running: Arc<AtomicBool>) {
    running.store(false, Ordering::SeqCst);
}

fn handle_packet(packet: &EthernetPacket, rules_option: &Option<Vec<rules::model::Rule>>) {
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
        // Handle the case where no rules are defined
        println!(
            "No rules defined, default action: Packet allowed: {:?}",
            packet
        );
    }
}
