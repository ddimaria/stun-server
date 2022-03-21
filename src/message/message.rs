use crate::error::{Error, Result};
use crate::message::attribute::Attribute;
use crate::message::class::Class;
use crate::message::method::Method;
use crate::message::transaction_id::TransactionId;
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// The magic cookie field MUST contain the fixed value 0x2112A442 in network
/// byte order. In [RFC3489], this field was part of the transaction ID;
/// placing the magic cookie in this location allows a server to detect if the
/// client will understand certain attributes that were added in this revised
/// specification. In addition, it aids in distinguishing STUN packets from
/// packets of other protocols when STUN is multiplexed with those other protocols
/// on the same port.
pub(crate) const MAGIC_COOKIE: u32 = 0x2112A442;

pub(crate) const MESSAGE_HEADER_LENGTH: usize = 20;

/// STUN messages are encoded in binary using network-oriented format (most
/// significant byte or octet first, also commonly known as big-endian). The
/// transmission order is described in detail in Appendix B of [RFC0791]. Unless
/// otherwise noted, numeric constants are in decimal (base 10).
///
/// All STUN messages MUST start with a 20-byte header followed by zero or more
/// Attributes. The STUN header contains a STUN message type, magic cookie,
/// transaction ID, and message length.
///
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0|     STUN Message Type     |         Message Length        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         Magic Cookie                          |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// |                     Transaction ID (96 bits)                  |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[derive(Debug, PartialEq)]
pub(crate) struct Message {
    pub(crate) class: Class,
    pub(crate) method: Method,
    pub(crate) transaction_id: TransactionId,
    pub(crate) attributes: Vec<Attribute>,
}

impl Message {
    pub(crate) fn binding_request(attributes: Vec<Attribute>) -> Message {
        Message {
            class: Class::Request,
            method: Method::Binding,
            transaction_id: TransactionId::new(),
            attributes,
        }
    }

    pub(crate) fn encode(&self, buf: &mut BytesMut) {
        let mut header: u16 = 0x0000;
        let transaction_id = &self.transaction_id.0;

        // encode message class
        header = self.class.encode(header);

        // encode message method
        header = self.method.encode(header);

        // add class and body to the buffer
        buf.put_u16(header);

        // encode the body length
        let body_bytes = BytesMut::with_capacity(256);
        let message_length: u16 = 0;
        for attribute in &self.attributes {
            // message_length += encode_attribute(attribute, &mut body_bytes, transaction_id);
        }

        // add message message length to the buffer
        buf.put_u16(message_length);

        // add magic cookie to the buffer
        buf.put_u32(MAGIC_COOKIE);

        // add transaction id to the buffer
        buf.put_slice(transaction_id);

        // body bytes
        buf.put_slice(body_bytes.as_ref());
    }

    pub(crate) fn decode(buffer: &mut Bytes) -> Result<Message> {
        let mut attributes: Vec<Attribute> = Vec::new();

        // All STUN messages MUST start with a 20-byte header followed by zero or
        // more Attributes. The STUN header contains a STUN message type, magic
        // cookie, transaction ID, and message length.
        if buffer.remaining() < MESSAGE_HEADER_LENGTH {
            return Err(Error::Decode(format!(
                "Not enough bytes in the header.  Expected {}, but got {}",
                20,
                buffer.remaining()
            )));
        }

        let message_type = buffer.get_u16();
        let class = Class::decode(message_type)?;
        let method = Method::decode(message_type);
        let message_length = buffer.get_u16() as usize;
        let magic_cookie = buffer.get_u32();

        // consumes 12 bytes from the buffer
        let transaction_id = TransactionId::decode(buffer)?;

        // validate magic cookie (the same for all stun messages)
        if magic_cookie != MAGIC_COOKIE {
            return Err(Error::Decode(format!(
                "Invalid magic cookie. Expected {}, but got {}.",
                MAGIC_COOKIE, magic_cookie
            )));
        }

        // only decode attributes if they exist
        if buffer.remaining() >= message_length {
            let reserved = buffer.remaining() - message_length;
            while buffer.remaining() > reserved {
                // let attribute = decode_attribute(buffer, &transaction_id_bytes)?;
                // attributes.push(attribute);
            }
        }

        // decode
        let msg = Message {
            class,
            method,
            transaction_id,
            attributes,
        };
        Result::Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BINDING_REQUEST: &[u8; 20] =
        b"\0\x01\0\0!\x12\xa4B\xb0\xb8?\0\xda\x0c\xa2\xc3(\xe1\xf2\x85";
    const BINDING_RESPONSE: &[u8; 20] = b"\x01\x01\0\0!\x12\xa4B\xc3>bhW \xc0\x8e\xd8\xf1y\x88";

    pub(crate) fn decode_message(buffer: &[u8; 20]) -> Message {
        let mut buffer = Bytes::copy_from_slice(buffer);
        Message::decode(&mut buffer).unwrap()
    }

    #[test]
    fn it_encodes_a_binding_request() {
        let mut buf = BytesMut::new();
        let message = Message {
            class: Class::Request,
            method: Method::Binding,
            transaction_id: TransactionId([176, 184, 63, 0, 218, 12, 162, 195, 40, 225, 242, 133]),
            attributes: vec![],
        };
        message.encode(&mut buf);

        let mut expected_buf = BytesMut::with_capacity(0);
        expected_buf.extend_from_slice(BINDING_REQUEST);

        assert_eq!(buf, expected_buf);
    }

    #[test]
    fn it_encodes_a_binding_response() {
        let mut buf = BytesMut::new();
        let message = Message {
            class: Class::SuccessResponse,
            method: Method::Binding,
            transaction_id: TransactionId([195, 62, 98, 104, 87, 32, 192, 142, 216, 241, 121, 136]),
            attributes: vec![],
        };
        message.encode(&mut buf);

        let mut expected_buf = BytesMut::with_capacity(0);
        expected_buf.extend_from_slice(BINDING_RESPONSE);

        assert_eq!(buf, expected_buf);
    }

    #[test]
    fn it_decodes_a_binding_request() {
        let message = decode_message(BINDING_REQUEST);
        let expected = Message {
            class: Class::Request,
            method: Method::Binding,
            transaction_id: TransactionId([176, 184, 63, 0, 218, 12, 162, 195, 40, 225, 242, 133]),
            attributes: vec![],
        };

        assert_eq!(message, expected);
    }

    #[test]
    fn it_decodes_a_binding_response() {
        let message = decode_message(BINDING_RESPONSE);
        let expected = Message {
            class: Class::SuccessResponse,
            method: Method::Binding,
            transaction_id: TransactionId([195, 62, 98, 104, 87, 32, 192, 142, 216, 241, 121, 136]),
            attributes: vec![],
        };

        assert_eq!(message, expected);
    }
}
