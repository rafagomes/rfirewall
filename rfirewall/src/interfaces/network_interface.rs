#[derive(Clone)]
pub struct NetworkInterfaceDetails {
    pub name: String,
    pub is_up: bool,
    pub is_loopback: bool,
}

pub trait NetworkInterface {
    fn get_interfaces(&self) -> Vec<NetworkInterfaceDetails>;
}
