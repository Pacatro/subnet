# Subnet

A library to create IPv4 subnetworks based on a given address and the number of hosts you want to connect.

## ✏️ Usage

```rust
use std::net::Ipv4Addr;

use subnet;
use subnet::SubnetData;

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
```

### Output

```terminal
Subnet address: 192.168.20.0
Broadcast: 192.168.20.127
Mask: /25
Useful range: [192.168.20.1 - 192.168.20.126]
```

## 📖 Add the library to your project

You can add the crate with `cargo add`

```terminal
cargo add subnet
```

Alternatively, you can manually add it to your project's Cargo.toml like this:

```toml
[dependencies]
subnet = "*" # Change the `*` to the current version
```

## 🔑 License

[MIT](https://opensource.org/license/mit/) - Created by **P4k0**.
