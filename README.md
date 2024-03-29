# Subnet

A library to create IPv4 subnetworks based on a given address and the number of hosts you want to connect.

## ✏️ Usage

```rust
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
```

### Output

```terminal
Subnet address: 192.168.20.0
Bin subnet: 11000000101010000001010000000000
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

## 💻 Contributing

If you want to add new features to the libray, you need to follow this steps.

Clone this repository

```terminal
git clone git@github.com:Pacatro/subnet.git
cd subnet
```

Run tests

```terminal
cargo test
```

Run example

```terminal
cargo run --example subnet
```

## 🔑 License

[MIT](https://opensource.org/license/mit/) - Created by [**P4k0**](https://github.com/Pacatro).
