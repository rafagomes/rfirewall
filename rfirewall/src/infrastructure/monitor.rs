use crate::core::packet_handler;
use crate::core::rule::Rule;
use pnet::packet::ethernet::EthernetPacket;

use crate::infrastructure::network::{
    create_datalink_channel, get_network_interfaces, select_interface,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Monitor {
    pub running: Arc<AtomicBool>,
}

impl Monitor {
    pub fn new(running: Arc<AtomicBool>) -> Self {
        Self { running }
    }

    pub fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&self) {
        start_monitor(self.running.clone());
    }

    pub fn stop(&self) {
        stop_monitor(self.running.clone());
    }
}

fn start_monitor(running: Arc<AtomicBool>) {
    let interfaces = get_network_interfaces();
    let interface = select_interface(&interfaces);

    let mut rx = create_datalink_channel(&interface);

    let rules_option = get_rules_option();

    monitor_loop(running, rx.as_mut(), &rules_option); // Dereference Box
}

pub fn stop_monitor(running: Arc<AtomicBool>) {
    running.store(false, Ordering::SeqCst);
}

fn monitor_loop(
    running: Arc<AtomicBool>,
    rx: &mut dyn pnet::datalink::DataLinkReceiver,
    rules_option: &Option<Vec<Rule>>,
) {
    println!("Monitoring started");
    while running.load(Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                let ethernet = EthernetPacket::new(packet).unwrap();
                packet_handler::handle_packet(&ethernet, &rules_option);
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::Interrupted {
                    eprintln!("Error reading packets: {}", e);
                } else {
                    break;
                }
            }
        }
    }
}

fn get_rules_option() -> Option<Vec<Rule>> {
    let rules = Rule::get_rules();
    if rules.is_empty() {
        None
    } else {
        Some(rules)
    }
}
