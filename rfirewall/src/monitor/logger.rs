use std::fs::OpenOptions;
use std::io::Write;

use pnet::packet::ethernet::EthernetPacket;

fn log_packet(action: &str, packet: &EthernetPacket) {
    let log_entry = format!("{:?}: Packet {:?} {}", chrono::Utc::now(), packet, action);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("firewall.log")
        .unwrap();

    writeln!(file, "{}", log_entry).unwrap();
}
