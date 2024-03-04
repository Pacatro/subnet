mod subnet_error;
mod addrs_data;
mod subnet_data;

use std::net::Ipv4Addr;
use subnet_error::SubnetError;
use addrs_data::AddrsData;

pub use subnet_data::SubnetData;

/// Converts the Ipv4 octets from decimal base to binary base
/// 
/// - `ip`: The IPv4 address
/// 
/// Returns a String with the octets in binary base.
/// 
/// ## Example
/// 
/// ```rust
/// let addrs = Ipv4Addr::new(192, 168, 20, 0);
/// assert_eq(ip_to_binary(addrs), "11000000101010000001010000000000");
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
/// use std::net::Ipv4Addr;
/// fn main() {
///     let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
///     let hosts: u32 = 120;
///
///     let subnet = subnet::create_subnet(addrs, hosts).unwrap();
///
///     let useful_range: Vec<Ipv4Addr> = subnet.get_useful_range();
///     assert_eq!(subnet.get_subnet_addrs().to_string(), "192.168.20.0");
///     assert_eq!(subnet.get_broadcast().to_string(), "192.168.20.127");
///     assert_eq!(subnet.get_mask(), 25);
///     assert_eq!(subnet.get_useful_range().len(), 126);
/// }
/// ```
///
pub fn create_subnet(addrs: Ipv4Addr, hosts: u32) -> Result<SubnetData, SubnetError> {
    let bin_ip: String = ip_to_binary(addrs);
    
    let addrs_data: AddrsData = AddrsData::build(addrs, bin_ip, hosts)?;

    let subnet: Ipv4Addr = addrs_data.get_addrs();
    let broadcast: Ipv4Addr = addrs_data.get_broadcast()?;
    let mask: u32 = addrs_data.get_mask()?;
    let useful_range: Vec<Ipv4Addr> = addrs_data.get_useful_range(&subnet, &broadcast);

    Ok(SubnetData::new(subnet, mask, useful_range, broadcast))
}

// TODO: HACER TESTS