pub use super::ip::IPError;
use std::fmt::{Debug, Display};
use std::ops::Deref;

pub struct Address(u32);

impl Deref for Address {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Address {
    pub fn new(ip: &str) -> Result<Self, IPError> {
        let octects: Vec<&str> = ip.split('.').collect();

        if octects.len() != 4 {
            return Err(IPError::InvalidAddress);
        }

        let mut address_bits: String = String::from("");

        for octect in octects {
            if octect.is_empty() {
                return Err(IPError::InvalidAddress);
            }

            let octect_as_int: u8;

            match octect.parse::<u8>() {
                Ok(o) => octect_as_int = o,
                Err(_) => return Err(IPError::InvalidAddress),
            }

            let octect_bits = format!("{:0>8b}", octect_as_int);
            address_bits.push_str(&octect_bits);
        }

        Ok(Self::from_radix(address_bits))
    }

    pub fn from(address: u32) -> Self {
        Self { 0: address }
    }

    pub fn from_radix(radix: String) -> Self {
        Self {
            0: u32::from_str_radix(&radix, 2).unwrap_or(0),
        }
    }

    pub fn as_str(&self) -> String {
        let mut shift = 24;
        let masks: Vec<Address> = vec![
            Address::new("255.0.0.0").unwrap(),
            Address::new("0.255.0.0").unwrap(),
            Address::new("0.0.255.0").unwrap(),
            Address::new("0.0.0.255").unwrap(),
        ];
        let mut ip_result = String::new();

        for i in 0..4 {
            let shifted: u8 = ((self.0 & *masks[i]) >> shift)
                .try_into()
                .expect("Error converting to u8");

            ip_result.push_str(&format!("{}", &shifted).to_string());

            if i != 3 {
                ip_result.push('.');
            }
            shift -= 8;
        }

        ip_result
    }

    pub fn bits(&self) -> String {
        format!("{:0>32b}", &self.0)
    }
}
