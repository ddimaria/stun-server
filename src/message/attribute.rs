use std::convert::{TryFrom, TryInto};

use crate::error::Result;
use crate::message::transaction_id::TransactionId;
use crate::utils::Address;
use bytes::{Buf, Bytes, BytesMut};

/// After the STUN header are zero or more attributes. Each attribute MUST be
/// TLV encoded, with a 16-bit type, 16-bit length, and value. Each STUN
/// attribute MUST end on a 32-bit boundary. As mentioned above, all fields
/// in an attribute are transmitted most significant bit first.
///
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |         Type                  |            Length             |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |                         Value (variable)                ....
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
/// The value in the length field MUST contain the length of the Value part of
/// the attribute, prior to padding, measured in bytes. Since STUN aligns attributes
/// on 32-bit boundaries, attributes whose content is not a multiple of 4 bytes
/// are padded with 1, 2, or 3 bytes of padding so that its value contains a multiple
/// of 4 bytes. The padding bits are ignored, and may be any value.
///
/// Any attribute type MAY appear more than once in a STUN message. Unless specified
/// otherwise, the order of appearance is significant: only the first occurrence needs
/// to be processed by a receiver, and any duplicates MAY be ignored by a receiver.
///
/// To allow future revisions of this specification to add new attributes if needed,
/// the attribute space is divided into two ranges. Attributes with type values between
/// 0x0000 and 0x7FFF are comprehension-required attributes, which means that the STUN
/// agent cannot successfully process the message unless it understands the attribute.
/// Attributes with type values between 0x8000 and 0xFFFF are comprehension-optional
/// attributes, which means that those attributes can be ignored by the STUN agent if
/// it does not understand them.
///
/// The set of STUN attribute types is maintained by IANA. The initial set defined by
/// this specification is found in Section 17.3.
///
/// The rest of this section describes the format of the various attributes defined
/// in this specification.
#[derive(Eq, PartialEq, Debug)]
pub(crate) enum Attribute {
    Username(String),
    Password(String),
    ErrorCode { code: u32, reason: String },
    FingerPrint(String),
    XorMappedAddress(Address),
    UnknownAttributes(Vec<u16>),
}

impl From<&mut BytesMut> for Attribute {
    fn from(buf: &mut BytesMut) -> Attribute {
        let code = buf.get_u16();
        let _message_length = buf.get_u16();
        let value_32: Vec<u8> = [
            buf.get_u8().to_be_bytes(),
            buf.get_u8().to_be_bytes(),
            buf.get_u8().to_be_bytes(),
            buf.get_u8().to_be_bytes(),
        ]
        .concat();
        let value = String::from_utf8(value_32).unwrap();

        match code {
            0x0006 => Attribute::Username(value),
            0x0007 => Attribute::Password(value),
            0x0009 => Attribute::ErrorCode {
                code: 0,
                reason: value,
            },
            0x000A => Attribute::UnknownAttributes(vec![code]),
            0x0020 => Attribute::XorMappedAddress(Address::try_from(value).unwrap()),
            0x8028 => Attribute::FingerPrint(value),
            _ => Attribute::UnknownAttributes(vec![code]),
        }
    }
}

impl Attribute {
    pub(crate) fn encode(&self, buf: &mut BytesMut, transaction_id: &TransactionId) -> u16 {
        0
    }

    pub(crate) fn decode(buf: &Bytes, transaction_id: &TransactionId) -> Result<Self> {
        Ok(Attribute::UnknownAttributes(vec![]))
    }
}
