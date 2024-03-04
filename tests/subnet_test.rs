use std::net::Ipv4Addr;

use subnet;

#[test]
fn test_subnet() {
    let subnet = subnet::create_subnet(Ipv4Addr::new(192, 168, 20, 0), 120);

    assert!(subnet.is_ok());
    
    let subnet = subnet.unwrap();

    assert_eq!(subnet.subnet_addrs().to_string(), "192.168.20.0");
    assert_eq!(subnet.broadcast().to_string(), "192.168.20.127");
    assert_eq!(subnet.mask(), 25);
    assert_eq!(subnet.useful_range().len(), 126);

    let subnet = subnet::create_subnet(Ipv4Addr::new(192, 168, 20, 0), 0);

    assert!(subnet.is_err());
    let err = subnet.unwrap_err();
    assert_eq!(err.to_string(), "Number of hosts must be greater than 0");
}