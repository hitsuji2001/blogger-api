use crate::errors::Error;

use std::net::{SocketAddr, ToSocketAddrs};

pub struct ServerConfig {
    pub address: SocketAddr,
}

impl ServerConfig {
    pub fn parse_from_env_file() -> Result<Self, Error> {
        let address = format!(
            "{}:{}",
            std::env::var("SERVER_HOST").expect("SERVER_HOST must be set"),
            std::env::var("SERVER_PORT").expect("SERVER_PORT must be set"),
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
