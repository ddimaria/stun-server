use crate::error::{Error, Result};
use std::{convert::TryFrom, net::SocketAddr};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Address {
    pub address: Vec<u8>,
    pub port: u16,
    pub ip_kind: IPKind,
}

impl Address {
    pub(crate) fn ipv4(address: [u8; 4], port: u16) -> Address {
        Address {
            address: address.to_vec(),
            port,
            ip_kind: IPKind::IPv4,
        }
    }

    pub(crate) fn ipv6(address: [u8; 16], port: u16) -> Address {
        Address {
            address: address.to_vec(),
            port,
            ip_kind: IPKind::IPv6,
        }
    }

    pub(crate) fn parse_address(socket_addr: SocketAddr) -> Address {
        match socket_addr {
            SocketAddr::V4(address) => Address::ipv4(address.ip().octets(), address.port()),
            SocketAddr::V6(address) => Address::ipv6(address.ip().octets(), address.port()),
        }
    }
}

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Address> {
        let address: SocketAddr = value.parse()?;
        Ok(Address::parse_address(address))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum IPKind {
    IPv4,
    IPv6,
}
