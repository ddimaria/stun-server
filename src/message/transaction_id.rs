use crate::error::{Error, Result};
use bytes::{Buf, Bytes};
use rand::Rng;
use std::convert::TryInto;

/// The transaction ID is a 96-bit identifier, used to uniquely identify STUN
/// transactions. For request/response transactions, the transaction ID is
/// chosen by the STUN client for the request and echoed by the server in the
/// response. For indications, it is chosen by the agent sending the indication.
/// It primarily serves to correlate requests with responses, though it also
/// plays a small role in helping to prevent certain types of attacks. The
/// server also uses the transaction ID as a key to identify each transaction
/// uniquely across all clients. As such, the transaction ID MUST be uniformly
/// and randomly chosen from the interval 0 .. 2**96-1, and SHOULD be
/// cryptographically random. Resends of the same request reuse the same
/// transaction ID, but the client MUST choose a new transaction ID for new
/// transactions unless the new request is bit-wise identical to the previous
/// request and sent from the same transport address to the same IP address. Success
/// and error responses MUST carry the same transaction ID as their corresponding
/// request. When an agent is acting as a STUN server and STUN client on the same
/// port, the transaction IDs in requests sent by the agent have no relationship to
/// the transaction IDs in requests received by the agent.
#[derive(Debug, PartialEq)]
pub(crate) struct TransactionId(pub [u8; 12]);

impl TransactionId {
    pub(crate) fn new() -> Self {
        Self(Self::random())
    }

    pub(crate) fn decode(buffer: &mut Bytes) -> Result<Self> {
        let transaction_id = buffer
            .get(0..12)
            .ok_or_else(|| {
                Error::Decode(
                    "not enough bytes (12) in the buffer allocated to the transaction id".into(),
                )
            })?
            .try_into()
            .map_err(|_| Error::Decode("could not decode the transaction id".into()))?;

        Ok(Self(transaction_id))
    }

    pub(crate) fn random() -> [u8; 12] {
        let mut transaction_id = [0u8; 12];
        rand::thread_rng().fill(&mut transaction_id[..]);
        transaction_id
    }
}
