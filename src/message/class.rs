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

use crate::error::{Error, Result};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub enum Class {
    Request,
    Indication,
    SuccessResponse,
    FailureResponse,
}

impl Class {
    pub(crate) fn encode(&self) -> u16 {
        let class: u16 = self.into();
        let class_part_0 = (class & 0x1) << 4; // C0
        let class_part_1 = (class & 0x2) << 7; // C1

        class_part_0 + class_part_1
    }

    pub(crate) fn decode(value: u16) -> Result<Self> {
        let class_part_0 = (value >> 4) & 0x1; // C0
        let class_part_1 = (value >> 7) & 0x2; // C1
        let class = class_part_0 + class_part_1;

        class.try_into()
    }
}

impl TryFrom<u16> for Class {
    type Error = Error;

    fn try_from(value: u16) -> Result<Class> {
        match value {
            0b00 => Ok(Class::Request),
            0b01 => Ok(Class::Indication),
            0b10 => Ok(Class::SuccessResponse),
            0b11 => Ok(Class::FailureResponse),
            _ => Err(Error::Parse(format!(
                "Could not convert {} to a message class",
                value
            ))),
        }
    }
}

impl From<&Class> for u16 {
    fn from(class: &Class) -> u16 {
        match class {
            Class::Request => 0b00,
            Class::Indication => 0b01,
            Class::SuccessResponse => 0b10,
            Class::FailureResponse => 0b11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BINGING_REQUEST: u16 = 0b000000000;
    const BINGING_INDICATION_RESPONSE: u16 = 0b000010000;
    const BINGING_SUCCESS: u16 = 0b100000000;
    const BINGING_FAILURE_RESPONSE: u16 = 0b100010000;

    #[test]
    fn it_encodes_all_classes() {
        let encoded = Class::Request.encode();
        assert_eq!(encoded, BINGING_REQUEST);

        let encoded = Class::Indication.encode();
        assert_eq!(encoded, BINGING_INDICATION_RESPONSE);

        let encoded = Class::SuccessResponse.encode();
        assert_eq!(encoded, BINGING_SUCCESS);

        let encoded = Class::FailureResponse.encode();
        assert_eq!(encoded, BINGING_FAILURE_RESPONSE);
    }

    #[test]
    fn it_decodes_all_classes() {
        let decoded = Class::decode(BINGING_REQUEST).unwrap();
        assert_eq!(decoded, Class::Request);

        let decoded = Class::decode(BINGING_INDICATION_RESPONSE).unwrap();
        assert_eq!(decoded, Class::Indication);

        let decoded = Class::decode(BINGING_SUCCESS).unwrap();
        assert_eq!(decoded, Class::SuccessResponse);

        let decoded = Class::decode(BINGING_FAILURE_RESPONSE).unwrap();
        assert_eq!(decoded, Class::FailureResponse);
    }
}
