use std::net::Ipv4Addr;

use subnet::{self, SubnetData};

fn main() {
    let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
    let hosts: u32 = 120;

    let subnet: SubnetData = subnet::create_subnet(addrs, hosts).unwrap_or_else(|err| {
        println!("Error: {}", err);
        std::process::exit(1);
    });

    let useful_range: Vec<Ipv4Addr> = subnet.get_useful_range();

    println!(
        "Subnet address: {}\nBroadcast: {}\nMask: /{}\nUseful range: [{} - {}]",
        subnet.get_subnet_addrs(), subnet.get_broadcast(), subnet.get_mask(), 
        useful_range.first().unwrap(), useful_range.last().unwrap()
    );
}