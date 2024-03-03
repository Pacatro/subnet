mod subnet_error;

use std::net::Ipv4Addr;
use subnet_error::SubnetError;

#[derive(PartialEq, Eq, Debug)]
struct AddrsData  {
    addrs: Ipv4Addr,
    bin_ip: String,
    host_bits: usize,
}

fn ip_to_binary(ip: Ipv4Addr) -> String {
    ip.octets()
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect::<Vec<String>>()
        .join("")
}

fn get_addrs_data(addrs: Ipv4Addr, hosts: u32) -> Result<AddrsData, SubnetError> {
    if hosts == 0 {
        return Err(SubnetError::InvalidNumberHosts("Number of hosts must be greater than 0".to_string()));
    }
    
    let mut host_bits: usize = 0;
    let mut num_addrs: u32 = 2;

    while num_addrs-2 < hosts {
        host_bits += 1;
        num_addrs = 2u32.pow(host_bits as u32);
    }

    let bin_ip: String = ip_to_binary(addrs);

    Ok(AddrsData { addrs, bin_ip, host_bits })
}

/// This struct contains all the information about the subnetwork
///
/// ## Atributes
/// - `subnet`: The subnet address
/// - `mask`: The subnet mask
/// - `useful_range`: The range of addresses that can be used by hosts
/// - `broadcast`: The broadcast address
///
#[derive(PartialEq, Eq, Debug)]
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

    pub fn get_subnet_addrs(&self) -> Ipv4Addr {
        self.subnet
    }

    pub fn get_mask(&self) -> u32 {
        self.mask
    }

    pub fn get_useful_range(&self) -> Vec<Ipv4Addr> {
        self.useful_range.clone()
    }

    pub fn get_broadcast(&self) -> Ipv4Addr {
        self.broadcast
    }
}

fn get_broadcast(addrs_data: &AddrsData) -> Result<Ipv4Addr, SubnetError> {
    let start_host_idx: usize = addrs_data.bin_ip.len() - addrs_data.host_bits;
    let host_bits: &str = &addrs_data.bin_ip[start_host_idx..];
    let host: u8 = u8::from_str_radix(&host_bits.replace("0", "1"), 2)
        .map_err(|err| SubnetError::ParserError(err.to_string()))?;

    let octecs: [u8; 4] = addrs_data.addrs.octets();

    Ok(Ipv4Addr::new(octecs[0], octecs[1], octecs[2], host))
}

fn get_mask(addrs_data: &AddrsData) -> Result<u32, SubnetError> {
    let start_host_idx: usize = addrs_data.bin_ip.len() - addrs_data.host_bits;
    let subnet_bits: &str = &addrs_data.bin_ip[..start_host_idx];

    Ok(subnet_bits.len() as u32)
}

fn get_useful_range(subnet_addrs: &Ipv4Addr, broadcast: &Ipv4Addr) -> Vec<Ipv4Addr> {
    (subnet_addrs.octets()[3]+1..broadcast.octets()[3])
        .map(|i| Ipv4Addr::new(subnet_addrs.octets()[0], subnet_addrs.octets()[1], subnet_addrs.octets()[2], i))
        .collect()
}

/// Create a subnet with the given base IP address and number of hosts
///
/// - `addrs`: The base IP address of the subnet
/// - `hosts`: The number of hosts in the subnet
///
/// Returns a SubnetData struct
///
/// ## Example
/// ```
/// use subnet::{create_subnet, SubnetData}
///
/// let subnet_data: SubnetData = create_subnet("192.168.0.1", 8).unwrap();
/// assert_eq!(subnet_data.subnet, "192.168.0.0");
/// assert_eq!(subnet_data.broadcast, "192.168.0.255");
/// assert_eq!(subnet_data.mask, 24);
/// assert_eq!(subnet_data.useful_range, vec!["192.168.0.1", "192.168.0.2", "192.168.0.3", "192.168.0.4", "192.168.0.5", "192.168.0.6"]);
/// ```
///
pub fn create_subnet(addrs: Ipv4Addr, hosts: u32) -> Result<SubnetData, SubnetError> {
    let addrs_data: AddrsData = get_addrs_data(addrs, hosts)?;

    let subnet: Ipv4Addr = addrs_data.addrs;
    let broadcast: Ipv4Addr = get_broadcast(&addrs_data)?;
    let mask: u32 = get_mask(&addrs_data)?;
    let useful_range: Vec<Ipv4Addr> = get_useful_range(&subnet, &broadcast);

    Ok(SubnetData { subnet, broadcast, mask, useful_range })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_ip_to_binary() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        assert_eq!(ip_to_binary(addrs), "11000000101010000001010000000000")
    }

    #[test]
    fn test_binary_ip_len() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        assert_eq!(ip_to_binary(addrs).len(), 32)
    }

    #[test]
    fn test_get_addrs_data() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let hosts: u32 = 120;

        let correct_ip_data: AddrsData = AddrsData { addrs, bin_ip: ip_to_binary(addrs), host_bits: 7 };

        assert_eq!(get_addrs_data(addrs, hosts).unwrap(), correct_ip_data)
    }

    #[test]
    fn test_get_broadcast() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let ip_data: AddrsData = AddrsData { addrs, bin_ip: ip_to_binary(addrs), host_bits: 7 };

        let broadcast: Result<Ipv4Addr, SubnetError> = get_broadcast(&ip_data);

        assert!(broadcast.is_ok());
        assert_eq!(broadcast.unwrap(), Ipv4Addr::new(192, 168, 20, 127))
    }

    #[test]
    fn test_get_mask() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let ip_data: AddrsData = AddrsData { addrs, bin_ip: ip_to_binary(addrs), host_bits: 7 };
        assert_eq!(get_mask(&ip_data).unwrap(), 25)
    }

    #[test]
    fn test_get_useful_range() {
        let subnet_ip: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let broadcast: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 127);

        let useful_range: Vec<Ipv4Addr> = get_useful_range(&subnet_ip, &broadcast);

        assert_eq!(useful_range.len(), 126)
    }

    #[test]
    fn test_create_subnet() {
        let subnet: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);

        let default_subnet_data: SubnetData = SubnetData::new(
            subnet,
            25,
            get_useful_range(&subnet, &Ipv4Addr::new(192, 168, 20, 127)),
            Ipv4Addr::new(192, 168, 20, 127)
        );

        assert_eq!(default_subnet_data, create_subnet(subnet, 120).unwrap());
    }
}