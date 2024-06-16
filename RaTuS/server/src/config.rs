use crate::error::Error;
use crate::result::Result;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub ip_address: std::net::IpAddr,
    pub port: u16,
    pub queue_size: usize,
}

impl Config {
    pub(crate) fn load_configuration(config_file_arg: Option<&str>) -> Result<Self> {
        if let Some(config_path) = config_file_arg {
            debug!("Loading config file: {}", config_path);
            Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
        } else {
            Err(Error::ConfigEmpty)
        }
    }
}
