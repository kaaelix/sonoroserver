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

pub const MAX_BODY_LEN: usize = 0x00FF_FFFF;

#[derive(Debug)]
pub enum MessageError {
    Truncated,
    InvalidMessageType(u8),
    ChecksumMismatch,
    FrameTooLarge(usize),
    Crypto(CryptoError),
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated message frame"),
            Self::InvalidMessageType(t) => write!(f, "invalid message type: {t}"),
            Self::ChecksumMismatch => write!(f, "CRC32 checksum mismatch"),
            Self::FrameTooLarge(len) => {
                write!(f, "encoded frame body length {len} exceeds 24-bit max ({MAX_BODY_LEN})")
            }
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

pub fn try_encode_raw_alloc(
    header: Header,
    raw_payload: &[u8],
    key: Option<&[u8; 32]>,
) -> Result<Vec<u8>, MessageError> {
    let hdr_len = match header {
        Header::Push(_) => 7usize,
        Header::Request(_) | Header::Response(_) => 9usize,
    };
    let min_body_len = hdr_len
        .saturating_add(4)
        .saturating_add(raw_payload.len());
    if min_body_len > MAX_BODY_LEN {
        return Err(MessageError::FrameTooLarge(min_body_len));
    }

    let encrypted;
    let wire_payload: &[u8] = if let Some(k) = key {
        encrypted = encryption::aes_encrypt_with_seq(raw_payload, k, header.seq_no());
        &encrypted
    } else {
        raw_payload
    };

    let body_len = hdr_len
        .saturating_add(4)
        .saturating_add(wire_payload.len());
    if body_len > MAX_BODY_LEN {
        return Err(MessageError::FrameTooLarge(body_len));
    }

    let mut hasher = Crc32Hasher::new();
    hasher.update(wire_payload);
    let checksum = hasher.finalize();

    let mut out = Vec::with_capacity(3 + body_len);
    out.extend_from_slice(&[0u8; 3]);
    header.encode(&mut out);
    out.extend_from_slice(&checksum.to_le_bytes());
    out.extend_from_slice(wire_payload);

    let actual_body_len = out.len() - 3;
    if actual_body_len > MAX_BODY_LEN {
        return Err(MessageError::FrameTooLarge(actual_body_len));
    }
    let len_bytes = (actual_body_len as u32).to_le_bytes();
    out[0..3].copy_from_slice(&len_bytes[0..3]);
    Ok(out)
}

pub fn encode_raw_alloc(header: Header, raw_payload: &[u8], key: Option<&[u8; 32]>) -> Vec<u8> {
    try_encode_raw_alloc(header, raw_payload, key)
        .unwrap_or_else(|err| panic!("encode_raw_alloc failed: {err}"))
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

    #[test]
    fn rejects_frames_exceeding_u24_body_length() {
        let header = Header::Push(PushHeader {
            seq_no: 1,
            msg_id: 100,
        });
        // Push header (7) + checksum (4) = 11 bytes overhead; MAX_BODY_LEN - 10 overflows 24-bit prefix.
        let oversized = vec![0u8; MAX_BODY_LEN - 10];
        assert!(matches!(
            try_encode_raw_alloc(header, &oversized, None),
            Err(MessageError::FrameTooLarge(_))
        ));

        // Encryption padding can also push a near-limit payload over MAX_BODY_LEN.
        let key = [0x11u8; 32];
        let near_limit = vec![0u8; MAX_BODY_LEN - 11];
        assert!(try_encode_raw_alloc(header, &near_limit, None).is_ok());
        assert!(matches!(
            try_encode_raw_alloc(header, &near_limit, Some(&key)),
            Err(MessageError::FrameTooLarge(_))
        ));
    }
}
