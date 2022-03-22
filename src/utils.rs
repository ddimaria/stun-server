use std::net::SocketAddr;

#[derive(Debug, Eq, PartialEq)]
pub struct Address {
    pub address: Vec<u8>,
    pub port: u16,
    pub ip_kind: IPKind,
}

// impl From<u16> for Attribute {
//     fn from(value: u16) -> Attribute {
//         match value {
//             0x0006 => Attribute::Username,
//             0x0007 => Attribute::Password,
//             0x0008 => Attribute::MessageIntegrity(),
//             0x0009 => Attribute::ErrorCode {
//                 code: 0,
//                 reason: "",
//             },
//             0x000A => Attribute::UnknownAttributes(),
//             0x0020 => Attribute::XorMappedAddress(),
//             0x8028 => Attribute::FingerPrint(),
//             _ => Attribute::UnknownAttributes(),
//         }
//     }
// }

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

#[derive(Debug, Eq, PartialEq)]
pub enum IPKind {
    IPv4,
    IPv6,
}
