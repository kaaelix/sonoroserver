use std::fmt;

use crc32fast::Hasher as Crc32Hasher;
use sonoro_proto::PbMessage;

use super::encryption::{self, CryptoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Request = 1,
    Response = 2,
    Push = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestHeader {
    pub seq_no: u32,
    pub rpc_id: u16,
    pub msg_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponseHeader {
    pub seq_no: u32,
    pub rpc_id: u16,
    pub msg_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushHeader {
    pub seq_no: u32,
    pub msg_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Header {
    Request(RequestHeader),
    Response(ResponseHeader),
    Push(PushHeader),
}

impl Header {
    pub fn decode(bytes: &[u8]) -> Result<(Self, usize), MessageError> {
        if bytes.is_empty() {
            return Err(MessageError::Truncated);
        }
        match bytes[0] {
            1 => {
                if bytes.len() < 9 {
                    return Err(MessageError::Truncated);
                }
                Ok((
                    Self::Request(RequestHeader {
                        seq_no: u32::from_le_bytes(bytes[1..5].try_into().unwrap()),
                        rpc_id: u16::from_le_bytes(bytes[5..7].try_into().unwrap()),
                        msg_id: u16::from_le_bytes(bytes[7..9].try_into().unwrap()),
                    }),
                    9,
                ))
            }
            2 => {
                if bytes.len() < 9 {
                    return Err(MessageError::Truncated);
                }
                Ok((
                    Self::Response(ResponseHeader {
                        seq_no: u32::from_le_bytes(bytes[1..5].try_into().unwrap()),
                        rpc_id: u16::from_le_bytes(bytes[5..7].try_into().unwrap()),
                        msg_id: u16::from_le_bytes(bytes[7..9].try_into().unwrap()),
                    }),
                    9,
                ))
            }
            4 => {
                if bytes.len() < 7 {
                    return Err(MessageError::Truncated);
                }
                Ok((
                    Self::Push(PushHeader {
                        seq_no: u32::from_le_bytes(bytes[1..5].try_into().unwrap()),
                        msg_id: u16::from_le_bytes(bytes[5..7].try_into().unwrap()),
                    }),
                    7,
                ))
            }
            other => Err(MessageError::InvalidMessageType(other)),
        }
    }

    pub fn encode(&self, out: &mut Vec<u8>) {
        match *self {
            Self::Request(h) => {
                out.push(MessageType::Request as u8);
                out.extend_from_slice(&h.seq_no.to_le_bytes());
                out.extend_from_slice(&h.rpc_id.to_le_bytes());
                out.extend_from_slice(&h.msg_id.to_le_bytes());
            }
            Self::Response(h) => {
                out.push(MessageType::Response as u8);
                out.extend_from_slice(&h.seq_no.to_le_bytes());
                out.extend_from_slice(&h.rpc_id.to_le_bytes());
                out.extend_from_slice(&h.msg_id.to_le_bytes());
            }
            Self::Push(h) => {
                out.push(MessageType::Push as u8);
                out.extend_from_slice(&h.seq_no.to_le_bytes());
                out.extend_from_slice(&h.msg_id.to_le_bytes());
            }
        }
    }

    pub fn seq_no(&self) -> u32 {
        match *self {
            Self::Request(h) => h.seq_no,
            Self::Response(h) => h.seq_no,
            Self::Push(h) => h.seq_no,
        }
    }

    pub fn message_id(&self) -> u16 {
        match *self {
            Self::Request(h) => h.msg_id,
            Self::Response(h) => h.msg_id,
            Self::Push(h) => h.msg_id,
        }
    }
}

#[derive(Debug)]
pub enum MessageError {
    Truncated,
    InvalidMessageType(u8),
    ChecksumMismatch,
    Crypto(CryptoError),
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated message frame"),
            Self::InvalidMessageType(t) => write!(f, "invalid message type: {t}"),
            Self::ChecksumMismatch => write!(f, "CRC32 checksum mismatch"),
            Self::Crypto(e) => write!(f, "crypto error: {e}"),
        }
    }
}

impl std::error::Error for MessageError {}

impl From<CryptoError> for MessageError {
    fn from(value: CryptoError) -> Self {
        Self::Crypto(value)
    }
}

pub struct Message {
    pub header: Header,
    pub payload: Vec<u8>,
}

impl Message {
    /// Decodes a body frame (after the 3-byte u24 length prefix has been stripped).
    pub fn decode(body: &[u8]) -> Result<Self, MessageError> {
        let (header, hdr_len) = Header::decode(body)?;
        if body.len() < hdr_len + 4 {
            return Err(MessageError::Truncated);
        }
        let checksum = u32::from_le_bytes(body[hdr_len..hdr_len + 4].try_into().unwrap());
        let payload = &body[hdr_len + 4..];

        let mut hasher = Crc32Hasher::new();
        hasher.update(payload);
        if hasher.finalize() != checksum {
            return Err(MessageError::ChecksumMismatch);
        }

        Ok(Self {
            header,
            payload: payload.to_vec(),
        })
    }

    pub fn decrypt(&mut self, key: &[u8; 32]) -> Result<(), MessageError> {
        self.payload = encryption::aes_decrypt_with_seq(&self.payload, key, self.header.seq_no())?;
        Ok(())
    }
}

pub fn encode_alloc<M: PbMessage>(header: Header, message: &M, key: Option<&[u8; 32]>) -> Vec<u8> {
    let raw_payload = message.encode_to_vec();
    encode_raw_alloc(header, &raw_payload, key)
}

pub fn encode_raw_alloc(header: Header, raw_payload: &[u8], key: Option<&[u8; 32]>) -> Vec<u8> {
    let encrypted;
    let wire_payload: &[u8] = if let Some(k) = key {
        encrypted = encryption::aes_encrypt_with_seq(raw_payload, k, header.seq_no());
        &encrypted
    } else {
        raw_payload
    };

    let mut hasher = Crc32Hasher::new();
    hasher.update(wire_payload);
    let checksum = hasher.finalize();

    let mut out = Vec::with_capacity(3 + 9 + 4 + wire_payload.len());
    out.extend_from_slice(&[0u8; 3]);
    header.encode(&mut out);
    out.extend_from_slice(&checksum.to_le_bytes());
    out.extend_from_slice(wire_payload);

    let body_len = (out.len() - 3) as u32;
    let len_bytes = body_len.to_le_bytes();
    out[0..3].copy_from_slice(&len_bytes[0..3]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use sonoro_proto::pb::JSPatchNotify;

    #[test]
    fn encodes_and_decodes_encrypted_push_frame() {
        let key = [0x5au8; 32];
        let notify = JSPatchNotify {
            Content: b"encrypted".to_vec(),
        };
        let frame = encode_alloc(
            Header::Push(PushHeader {
                seq_no: 2,
                msg_id: JSPatchNotify::MSG_ID,
            }),
            &notify,
            Some(&key),
        );
        let mut msg = Message::decode(&frame[3..]).expect("decode frame");
        msg.decrypt(&key).expect("decrypt frame");
        let decoded = JSPatchNotify::decode(&msg.payload).expect("decode pb");
        assert_eq!(decoded.Content, b"encrypted");
    }
}
