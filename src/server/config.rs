use crate::errors::Error;
use crate::utils::env;

use std::net::{SocketAddr, ToSocketAddrs};
pub struct ServerConfig {
    pub address: SocketAddr,
}

impl ServerConfig {
    pub fn parse_from_env_file(file_path: &str) -> Result<Self, Error> {
        let env = env::get_env_parser_from_file(file_path)?;
        let address = format!(
            "{}:{}",
            env::find_key_from_parser(&String::from("SERVER_HOST"), &env)?,
            env::find_key_from_parser(&String::from("SERVER_PORT"), &env)?,
        );
        let addresses: Vec<_> = address
            .to_socket_addrs()
            .map_err(|err| Error::ServerNoSuchIP(address.clone(), err.to_string()))?
            .collect();
        assert_eq!(
            addresses.len(),
            1,
            "Multiple server IP found: {}",
            address.len()
        );

        Ok(ServerConfig {
            address: addresses[0],
        })
    }
}
