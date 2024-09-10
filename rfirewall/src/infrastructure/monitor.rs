use crate::core::packet_handler;
use crate::core::rule::Rule;
use crate::interfaces::{NetworkInterface, NetworkInterfaceDetails};
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub fn start_monitor(running: Arc<AtomicBool>) {
    let interfaces = get_network_interfaces();
    let interface = select_interface(&interfaces);

    let mut rx = create_datalink_channel(&interface);

    let rules_option = get_rules_option();

    monitor_loop(running, rx.as_mut(), &rules_option); // Dereference Box
}

pub fn stop_monitor(running: Arc<AtomicBool>) {
    running.store(false, Ordering::SeqCst);
}

fn get_network_interfaces() -> Vec<datalink::NetworkInterface> {
    datalink::interfaces()
}

fn select_interface(interfaces: &[datalink::NetworkInterface]) -> datalink::NetworkInterface {
    interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No interfaces were found")
        .clone()
}

fn create_datalink_channel(
    interface: &datalink::NetworkInterface,
) -> Box<dyn pnet::datalink::DataLinkReceiver> {
    let (_, rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(_, rx)) => ((), rx), // Unpack the tuple and discard the transmitter (tx)
        Ok(_) => {
            eprintln!("Unhandled channel type");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error creating the datalink channel: {}", e);
            process::exit(1);
        }
    };
    rx
}

fn get_rules_option() -> Option<Vec<Rule>> {
    let rules = Rule::get_rules();
    if rules.is_empty() {
        None
    } else {
        Some(rules)
    }
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
