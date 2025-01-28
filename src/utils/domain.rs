use crate::ftp::error::{Error, Result};
use std::net::{SocketAddr, ToSocketAddrs};
use std::vec::IntoIter;

use super::validator::DomainAllowPort;

pub fn resolve_domain_to_socket_addr(addr: &DomainAllowPort) -> Result<SocketAddr> {
    let mut addrs: IntoIter<SocketAddr> = if let Some(port) = addr.port {
        (addr.domain.as_str(), port)
            .to_socket_addrs()
            .map_err(|_| Error::DomainResolutionError)?
    } else {
        (addr.domain.as_str(), 21)
            .to_socket_addrs()
            .map_err(|_| Error::DomainResolutionError)?
    };

    if let Some(addr) = addrs.next() {
        Ok(addr)
    } else {
        Err(Error::DomainResolutionError)
    }
}
