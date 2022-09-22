//! The message type field is decomposed further into the following structure:
//!
//! 0                 1
//! 2  3  4 5 6 7 8 9 0 1 2 3 4 5
//! +--+--+-+-+-+-+-+-+-+-+-+-+-+-+
//! |M |M |M|M|M|C|M|M|M|C|M|M|M|M|
//! |11|10|9|8|7|1|6|5|4|0|3|2|1|0|
//! +--+--+-+-+-+-+-+-+-+-+-+-+-+-+
//!
//! Here the bits in the message type field are shown as most significant (M11)
//! through least significant (M0). M11 through M0 represent a 12-bit encoding
//! of the method. C1 and C0 represent a 2-bit encoding of the class. A class
//! of 0b00 is a request, a class of 0b01 is an indication, a class of 0b10
//! is a success response, and a class of 0b11 is an error response. This
//! specification defines a single method, Binding. The method and class are
//! orthogonal, so that for each method, a request, success response, error response,
//! and indication are possible for that method. Extensions defining new methods
//! MUST indicate which classes are permitted for that method.

#[derive(Debug, PartialEq)]
pub enum Method {
    Binding,
}

impl Method {
    pub(crate) fn encode(&self) -> u16 {
        let method: u16 = self.into();
        let method = method & 0xFFF;
        let method_part_0_3 = method & 0x000F; // M0-M3
        let method_part_4_6 = method & 0x0070; // M4-M6
        let method_part_7_11 = method & 0x0F80; // M7-M11

        method_part_0_3 + method_part_4_6 + method_part_7_11
    }

    pub(crate) fn decode(value: u16) -> Self {
        let method_part_0_3 = value & 0xf; // M0-M3
        let method_part_4_6 = (value >> 1) & 0x70; // M4-M6
        let method_part_7_11 = (value >> 2) & 0xf80; // M7-M11
        let method = method_part_0_3 + method_part_4_6 + method_part_7_11;

        method.into()
    }
}

impl From<u16> for Method {
    fn from(value: u16) -> Method {
        match value {
            0x001 => Method::Binding,
            _ => unimplemented!("Only binding methods are allowed"),
        }
    }
}

impl From<&Method> for u16 {
    fn from(value: &Method) -> u16 {
        match value {
            Method::Binding => 0x001,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_all_methods() {
        let encoded = Method::Binding.encode();
        assert_eq!(encoded, (&Method::Binding).into());
    }

    #[test]
    fn it_decodes_all_methods() {
        let decoded = Method::decode((&Method::Binding).into());
        assert_eq!(decoded, Method::Binding);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_decoding_a_non_binding_method() {
        let method = 0x0002;
        let _: Method = method.into();
    }
}
