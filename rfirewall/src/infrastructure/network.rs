use pnet::datalink::{self, Channel::Ethernet};
use std::process;

pub fn get_network_interfaces() -> Vec<datalink::NetworkInterface> {
    datalink::interfaces()
}

pub fn select_interface(interfaces: &[datalink::NetworkInterface]) -> datalink::NetworkInterface {
    interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No interfaces were found")
        .clone()
}

pub fn create_datalink_channel(
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
