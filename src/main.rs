use std::net::Ipv4Addr;
use std::process;
use std::error::Error;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Subnet", author, version, about, long_about = None)]
struct Cli {
    /// IPv4 address
    addr: Ipv4Addr,

    /// Number of machines to implement subnet
    machines: u32
}

struct IpData  {
    ip: Ipv4Addr,
    bin_ip: String,
    host_bits: usize
}

struct SubnetData {
    subnet: Ipv4Addr,
    mask: u32,
    useful_range: Vec<Ipv4Addr>,
    broadcast: Ipv4Addr
}

fn ip_to_binary(ip: Ipv4Addr) -> String {
    let mut binary: Vec<String> = Vec::new();

    let octecs: [u8; 4] = ip.octets();

    for oct in octecs {
        let bin: String = format!("{oct:08b}");
        binary.push(bin);
    }

    binary.concat()
}

fn get_ip_data(ip: Ipv4Addr, disp: u32) -> IpData {
    let mut host_bits: usize = 0;
    let mut num: u32 = 2;

    while num-2 < disp {
        host_bits += 1;
        num = 2u32.pow(host_bits as u32);
    }

    let bin_ip: String = ip_to_binary(ip);

    IpData { ip, bin_ip, host_bits }
}

fn get_broadcast(ip_data: &IpData) -> Result<Ipv4Addr, Box<dyn Error>> {
    let start_host_idx: usize = ip_data.bin_ip.len() - ip_data.host_bits;
    let host_bits: &str = &ip_data.bin_ip[start_host_idx..];
    let host: u8 = u8::from_str_radix(&host_bits.replace("0", "1"), 2)?;

    let octecs: [u8; 4] = ip_data.ip.octets();

    Ok(Ipv4Addr::new(octecs[0], octecs[1], octecs[2], host))
}

fn get_mask(ip_data: &IpData) -> u32 {
    let start_host_idx: usize = ip_data.bin_ip.len() - ip_data.host_bits;
    let subnet_bits: &str = &ip_data.bin_ip[..start_host_idx];
    subnet_bits.len() as u32
}

fn get_useful_range(subnet_ip: &Ipv4Addr, broadcast: &Ipv4Addr) -> Vec<Ipv4Addr> {
    let mut addrs: Vec<Ipv4Addr> = Vec::new();

    let subnet_octecs: [u8; 4] = subnet_ip.octets();
    let broadcast_octecs: [u8; 4] = broadcast.octets();

    for i in subnet_octecs[3]+1..broadcast_octecs[3] {
        addrs.push(Ipv4Addr::new(subnet_octecs[0], subnet_octecs[1], subnet_octecs[2], i));
    }
    
    addrs
}

fn get_subnet_data(ip_data: IpData) -> Result<SubnetData, Box<dyn Error>> {
    let subnet: Ipv4Addr = ip_data.ip;
    let broadcast: Ipv4Addr = get_broadcast(&ip_data)?;
    let mask: u32 = get_mask(&ip_data);
    let useful_range: Vec<Ipv4Addr> = get_useful_range(&subnet, &broadcast);

    Ok(SubnetData { subnet, broadcast, mask, useful_range })
}

fn main() {
    let cli: Cli = Cli::parse();

    let ip_data: IpData = get_ip_data(cli.addr, cli.machines);

    let subnet_data: SubnetData = get_subnet_data(ip_data).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(-1);
    });

    println!(
        "Subnet: {} | Broadcast: {} | Mask: /{} | Useful range: [{} - {}]",
        subnet_data.subnet, subnet_data.broadcast, subnet_data.mask, 
        subnet_data.useful_range.first().unwrap(), subnet_data.useful_range.last().unwrap()
    );
}
