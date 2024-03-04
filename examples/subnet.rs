use std::net::Ipv4Addr;

use subnet::{self, SubnetData, SubnetError};

fn main() {
    let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
    let hosts: u32 = 120;

    let subnet: SubnetData = subnet::create_subnet(addrs, hosts).unwrap_or_else(|err: SubnetError| {
        println!("Error: {}", err);
        std::process::exit(1);
    });

    let useful_range: Vec<Ipv4Addr> = subnet.useful_range();

    let bin_subnet: String = subnet::ip_to_binary(subnet.subnet_addrs());

    println!(
        "Subnet address: {}\nBin subnet: {}\nBroadcast: {}\nMask: /{}\nUseful range: [{} - {}]",
        subnet.subnet_addrs(), bin_subnet, subnet.broadcast(), subnet.mask(), 
        useful_range.first().unwrap(), useful_range.last().unwrap()
    );
}