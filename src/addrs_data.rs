use std::net::Ipv4Addr;
use crate::subnet_error::SubnetError;


#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct AddrsData  {
    addrs: Ipv4Addr,
    bin_ip: String,
    host_bits: u32,
}

impl AddrsData {
    fn new(addrs: Ipv4Addr, bin_ip: String, host_bits: u32) -> Self {
        Self { addrs, bin_ip, host_bits }
    }

    pub(crate) fn addrs(&self) -> Ipv4Addr {
        self.addrs
    }

    fn bin_ip(&self) -> &str {
        &self.bin_ip
    }

    fn host_bits(&self) -> u32 {
        self.host_bits
    }

    pub(crate) fn build(addrs: Ipv4Addr, bin_ip: String, hosts: u32) -> Result<AddrsData, SubnetError> {
        if hosts == 0 {
            return Err(SubnetError::InvalidNumberHosts("Number of hosts must be greater than 0".to_string()));
        }
        
        let mut host_bits: u32 = 0;
        let mut num_addrs: u32 = 2;
    
        while num_addrs-2 < hosts {
            host_bits += 1;
            num_addrs = 2u32.pow(host_bits as u32);
        }
    
        Ok(AddrsData::new(addrs, bin_ip, host_bits))
    }

    pub(crate) fn get_broadcast(&self) -> Result<Ipv4Addr, SubnetError> {
        let start_host_idx: usize = self.bin_ip().len() - (self.host_bits() as usize);
        let host_bits: &str = &self.bin_ip()[start_host_idx..];
        let host: u8 = u8::from_str_radix(&host_bits.replace("0", "1"), 2)
            .map_err(|err| SubnetError::ParserError(err.to_string()))?;
    
        let octecs: [u8; 4] = self.addrs.octets();
    
        Ok(Ipv4Addr::new(octecs[0], octecs[1], octecs[2], host))
    }

    pub(crate) fn get_mask(&self) -> Result<u32, SubnetError> {
        let start_host_idx: usize = self.bin_ip().len() - (self.host_bits() as usize);
        let subnet_bits: &str = &self.bin_ip()[..start_host_idx];
    
        Ok(subnet_bits.len() as u32)
    }

    pub(crate) fn get_useful_range(&self) -> Result<Vec<Ipv4Addr>, SubnetError> {
        let broadcast: Ipv4Addr = self.get_broadcast()?;
        Ok((self.addrs.octets()[3]+1..broadcast.octets()[3])
            .map(|i| Ipv4Addr::new(self.addrs.octets()[0], self.addrs.octets()[1], self.addrs.octets()[2], i))
            .collect())
    }
}

#[cfg(test)]
mod test {
    use crate::ip_to_binary;

    use super::{AddrsData, SubnetError};
    use std::{net::Ipv4Addr, str::FromStr};

    #[test]
    fn test_buil_addrs_data() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let bin_ip: String = ip_to_binary(addrs);
        let hosts: u32 = 120;
        let host_bits: u32 = 7;

        let addrs_data: AddrsData = AddrsData::new(addrs, bin_ip.clone(), host_bits);
        let result: Result<AddrsData, SubnetError> = AddrsData::build(addrs, bin_ip.clone(), hosts);

        assert!(result.is_ok());

        let new_addrs_data: AddrsData = result.unwrap();

        assert_eq!(addrs_data.addrs(), new_addrs_data.addrs());
        assert_eq!(addrs_data.bin_ip(), new_addrs_data.bin_ip());
        assert_eq!(addrs_data.host_bits(), new_addrs_data.host_bits());
    }

    #[test]
    fn test_get_broadcast() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let bin_ip: String = ip_to_binary(addrs);
        let host_bits: u32 = 7;

        let addrs_data: AddrsData = AddrsData::new(addrs, bin_ip.clone(), host_bits);

        let result: Result<Ipv4Addr, SubnetError> = addrs_data.get_broadcast();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Ipv4Addr::new(192, 168, 20, 127));
    }

    #[test]
    fn test_get_mask() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let bin_ip: String = ip_to_binary(addrs);
        let host_bits: u32 = 7;

        let addrs_data: AddrsData = AddrsData::new(addrs, bin_ip.clone(), host_bits);

        let result: Result<u32, SubnetError> = addrs_data.get_mask();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 25);
    }

    #[test]
    fn test_get_useful_range() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 0);
        let bin_ip: String = ip_to_binary(addrs);
        let host_bits: u32 = 3;

        let addrs_data: AddrsData = AddrsData::new(addrs, bin_ip.clone(), host_bits);

        let result: Result<Vec<Ipv4Addr>, SubnetError> = addrs_data.get_useful_range();

        let good_range: Vec<Ipv4Addr> = vec![Ipv4Addr::from_str("192.168.0.1").unwrap(), 
                                             Ipv4Addr::from_str("192.168.0.2").unwrap(), 
                                             Ipv4Addr::from_str("192.168.0.3").unwrap(), 
                                             Ipv4Addr::from_str("192.168.0.4").unwrap(), 
                                             Ipv4Addr::from_str("192.168.0.5").unwrap(), 
                                             Ipv4Addr::from_str("192.168.0.6").unwrap()];

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), good_range);
    }
}