use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub ip_address: std::net::IpAddr,
    pub port: u16,
    pub queue_size: usize,
}

impl Config {
    pub fn new<T>(ip_address: T, port: u16, queue_size: usize) -> Self 
    where T: AsRef<str>
    {
        use std::net::IpAddr;
        use std::str::FromStr;
    
        let _ip_address = IpAddr::from_str(ip_address.as_ref()).expect("Invalid IP address");
        Self {
            ip_address: _ip_address,
            port,
            queue_size,
        }
    }
}
