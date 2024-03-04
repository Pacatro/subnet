mod subnet_error;
mod addrs_data;
mod subnet_data;

use std::net::Ipv4Addr;
use addrs_data::AddrsData;

pub use subnet_data::SubnetData;
pub use subnet_error::SubnetError;

/// Converts the Ipv4 octets from decimal base to binary base
/// 
/// - `ip`: The IPv4 address
/// 
/// Returns a String with the octets in binary base.
/// 
/// ## Example
/// 
/// ```rust
/// let addrs = std::net::Ipv4Addr::new(192, 168, 20, 0);
/// assert_eq!(subnet::ip_to_binary(addrs), "11000000101010000001010000000000");
/// ```
/// 
pub fn ip_to_binary(ip: Ipv4Addr) -> String {
    ip.octets()
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect::<Vec<String>>()
        .join("")
}

/// Create a subnet with the given base IP address and number of hosts
///
/// - `addrs`: The base IP address of the subnet
/// - `hosts`: The number of hosts in the subnet
///
/// Returns a SubnetData struct
///
/// ## Example
///
/// ```rust
/// 
/// fn main() {
///     let addrs = std::net::Ipv4Addr::new(192, 168, 20, 0);
///     let hosts= 120;
///
///     let subnet = subnet::create_subnet(addrs, hosts).unwrap();
///
///     let useful_range = subnet.useful_range();
///     assert_eq!(subnet.subnet_addrs().to_string(), "192.168.20.0");
///     assert_eq!(subnet.broadcast().to_string(), "192.168.20.127");
///     assert_eq!(subnet.mask(), 25);
///     assert_eq!(subnet.useful_range().len(), 126);
/// }
/// ```
///
pub fn create_subnet(addrs: Ipv4Addr, hosts: u32) -> Result<SubnetData, SubnetError> {
    let bin_ip: String = ip_to_binary(addrs);
    
    let addrs_data: AddrsData = AddrsData::build(addrs, bin_ip, hosts)?;

    let subnet: Ipv4Addr = addrs_data.addrs();
    let broadcast: Ipv4Addr = addrs_data.get_broadcast()?;
    let mask: u32 = addrs_data.get_mask()?;
    let useful_range: Vec<Ipv4Addr> = addrs_data.get_useful_range()?;

    Ok(SubnetData::new(subnet, mask, useful_range, broadcast))
}

// TODO: HACER TESTS
#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_ip_to_binary() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        assert_eq!(ip_to_binary(addrs), "11000000101010000001010000000000");
    }

    #[test]
    fn test_create_subnet() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let hosts: u32 = 120;
        let result: Result<SubnetData, SubnetError> = create_subnet(addrs, hosts);

        assert!(result.is_ok());

        let subnet: SubnetData = result.unwrap();

        assert_eq!(subnet.subnet_addrs(), Ipv4Addr::new(192, 168, 20, 0));
        assert_eq!(subnet.broadcast(), Ipv4Addr::new(192, 168, 20, 127));
        assert_eq!(subnet.mask(), 25);
        assert_eq!(subnet.useful_range().len(), 126);
    }
}