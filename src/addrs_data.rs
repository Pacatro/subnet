use std::net::Ipv4Addr;
use crate::subnet_error::SubnetError;


#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct AddrsData  {
    addrs: Ipv4Addr,
    bin_ip: String,
    host_bits: u32,
}

impl AddrsData {
    pub(crate) fn new(addrs: Ipv4Addr, bin_ip: String, host_bits: u32) -> Self {
        Self { addrs, bin_ip, host_bits }
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
    
        Ok(AddrsData { addrs, bin_ip, host_bits })
    }

    pub(crate) fn get_addrs(&self) -> Ipv4Addr {
        self.addrs
    }

    pub(crate) fn get_broadcast(&self) -> Result<Ipv4Addr, SubnetError> {
        let start_host_idx: usize = self.bin_ip.len() - (self.host_bits as usize);
        let host_bits: &str = &self.bin_ip[start_host_idx..];
        let host: u8 = u8::from_str_radix(&host_bits.replace("0", "1"), 2)
            .map_err(|err| SubnetError::ParserError(err.to_string()))?;
    
        let octecs: [u8; 4] = self.addrs.octets();
    
        Ok(Ipv4Addr::new(octecs[0], octecs[1], octecs[2], host))
    }

    pub(crate) fn get_mask(&self) -> Result<u32, SubnetError> {
        let start_host_idx: usize = self.bin_ip.len() - (self.host_bits as usize);
        let subnet_bits: &str = &self.bin_ip[..start_host_idx];
    
        Ok(subnet_bits.len() as u32)
    }

    pub(crate) fn get_useful_range(&self, subnet_addrs: &Ipv4Addr, broadcast: &Ipv4Addr) -> Vec<Ipv4Addr> {
        (subnet_addrs.octets()[3]+1..broadcast.octets()[3])
            .map(|i| Ipv4Addr::new(subnet_addrs.octets()[0], subnet_addrs.octets()[1], subnet_addrs.octets()[2], i))
            .collect()
    }
}

// TODO: HACER TESTS
#[cfg(test)]
mod test {
    use crate::ip_to_binary;

    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_buil_addrs_data() {
        let addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let bin_ip: String = ip_to_binary(addrs);
        let hosts = 120;
        let host_bits: u32 = 7;

        let addrs_data: AddrsData = AddrsData::new(addrs, bin_ip.clone(), host_bits);
        let result: Result<AddrsData, SubnetError> = AddrsData::build(addrs, bin_ip.clone(), hosts);

        assert!(result.is_ok());

        let new_addrs_data: AddrsData = result.unwrap();

        assert_eq!(addrs_data.get_addrs(), new_addrs_data.get_addrs());
        assert_eq!(addrs_data.get_broadcast().unwrap(), new_addrs_data.get_broadcast().unwrap());
        assert_eq!(addrs_data.get_mask().unwrap(), new_addrs_data.get_mask().unwrap());

        let subnet_addrs: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 0);
        let broadcast: Ipv4Addr = Ipv4Addr::new(192, 168, 20, 127);

        assert_eq!(addrs_data.get_useful_range(&subnet_addrs, &broadcast), new_addrs_data.get_useful_range(&subnet_addrs, &broadcast));
    }
}