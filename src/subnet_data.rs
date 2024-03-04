use std::net::Ipv4Addr;

/// This struct contains all the information about the subnetwork
///
/// ## Atributes
/// - `subnet`: The subnet address
/// - `mask`: The subnet mask
/// - `useful_range`: The range of addresses that can be used by hosts
/// - `broadcast`: The broadcast address
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubnetData {
    subnet: Ipv4Addr,
    mask: u32,
    useful_range: Vec<Ipv4Addr>,
    broadcast: Ipv4Addr
}

impl SubnetData {
    pub fn new(subnet: Ipv4Addr, mask: u32, useful_range: Vec<Ipv4Addr>, broadcast: Ipv4Addr) -> Self {
        Self { subnet, mask, useful_range, broadcast }
    }

    pub fn subnet_addrs(&self) -> Ipv4Addr {
        self.subnet
    }

    pub fn mask(&self) -> u32 {
        self.mask
    }

    pub fn useful_range(&self) -> Vec<Ipv4Addr> {
        self.useful_range.clone()
    }

    pub fn broadcast(&self) -> Ipv4Addr {
        self.broadcast
    }
}