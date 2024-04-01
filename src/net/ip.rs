use super::Address;
use super::Prefix;
use std::fmt::{Debug, Display};

pub struct IP {
    address: Address,
    mask: Address,
    net_address: Address,
    prefix: Prefix,
    is_network_address: bool,
}

impl IP {
    pub fn prefix(&self) -> &Prefix {
        &self.prefix
    }
    pub fn net_addr(&self) -> &Address {
        &self.net_address
    }
    pub fn mask(&self) -> &Address {
        &self.mask
    }
    pub fn broadcast(&self) -> Address {
        let mut addr_bits = self.net_address.bits().to_owned();
        let first_host_bit: usize = self.prefix.into();
        let hosts_bits: usize = (32 - &self.prefix).into();

        addr_bits.replace_range(first_host_bit.., &"1".repeat(hosts_bits));
        Address::from_radix(addr_bits)
    }
    pub fn last_host(&self) -> Address {
        let mut addr_bits = self.net_address.bits().to_owned();
        let first_host_bit: usize = self.prefix.into();
        let hosts_bits: usize = (32 - &self.prefix).into();

        addr_bits.replace_range(first_host_bit.., &"1".repeat(hosts_bits));
        addr_bits.replace_range(31.., "0");
        Address::from_radix(addr_bits)
    }
    pub fn first_host(&self) -> Address {
        let mut addr_bits = self.net_address.bits().to_owned();
        addr_bits.replace_range(31.., "1");
        Address::from_radix(addr_bits)
    }
    pub fn wildcard(&self) -> Address {
        let binding = self.mask.bits().to_owned();
        let subnet: String = binding
            .chars()
            .map(|x| match x {
                '0' => '1',
                '1' => '0',
                _ => ' ',
            })
            .collect();
        Address::from_radix(subnet)
    }

    pub fn subnet(&self, new_prefix: u8) -> Result<Vec<Self>, IPError> {
        if new_prefix <= self.prefix {
            return Err(IPError::InvalidSubnetPrefix);
        } else if !self.is_network_address {
            return Err(IPError::InvalidNetworkAddress);
        }

        let mut subnets: Vec<Self> = Vec::new();
        let mut bits = self.net_address.bits().to_owned();

        let borrowed_bits_count = new_prefix - self.prefix();
        let first_bit: usize = (self.prefix).into();
        let last_bit: usize = (self.prefix + borrowed_bits_count).into();
        let subnet_combs = u32::pow(2, borrowed_bits_count.into());

        for i in 0..subnet_combs {
            bits.replace_range(
                first_bit..last_bit,
                &format!("{:0>w$b}", i, w = borrowed_bits_count.into()),
            );

            let new_subnet = format!("{}/{}", Address::from_radix(bits.to_owned()), new_prefix);
            subnets.push(Self::try_from(&new_subnet as &str)?);
        }

        Ok(subnets)
    }
}

impl Debug for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}\nnetwork: {}\nsubnet mask: {}",
            &self.address, &self.prefix, &self.net_address, &self.mask
        )
    }
}

impl Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.address, &self.prefix)
    }
}

impl TryFrom<&str> for IP {
    type Error = IPError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let found = value.find('/');
        let i;
        match found {
            Some(idx) => i = idx,
            None => return Err(IPError::InvalidAddress),
        }

        let address: Address = Address::new(&value[..i])?;
        let prefix: Prefix;
        match value[i + 1..].parse::<Prefix>() {
            Ok(pref) => prefix = pref,
            Err(_) => return Err(IPError::InvalidAddress),
        }

        if prefix > 32 {
            return Err(IPError::InvalidPrefix);
        }

        let shift = 32 - prefix;
        let mask = Address::from(4294967295 << shift);

        let net_address = Address::from(*address & *mask);

        let is_network_address = *address == *net_address;

        Ok(IP {
            address,
            prefix,
            mask,
            net_address,
            is_network_address,
        })
    }
}

#[derive(Debug)]
pub enum IPError {
    InvalidPrefix,
    InvalidAddress,
    InvalidSubnetPrefix,
    InvalidNetworkAddress,
}

impl Display for IPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IPError::InvalidPrefix => write!(f, "Se esperaba un prefijo entre 0 y 32"),
            IPError::InvalidAddress => {
                write!(
                    f,
                    "Formato de direccion incorrecto, se esperaba x.x.x.x/x para 0 >= x <= 255"
                )
            }
            IPError::InvalidSubnetPrefix => write!(f, "El prefijo de subneteo es incorrecto"),
            IPError::InvalidNetworkAddress => {
                write!(f, "La direccion para subneteo debe ser de red")
            }
        }
    }
}
