//! Lightweight, zero-macro protobuf wire encoder and decoder matching `proto/src/root.zig`.

use std::collections::BTreeMap;
use std::fmt;

/// Protobuf wire types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl WireType {
    /// Converts the lower 3 bits of a wire tag into a [`WireType`].
    pub fn from_u32(v: u32) -> Result<Self, DecodeError> {
        match v & 7 {
            0 => Ok(Self::Varint),
            1 => Ok(Self::Fixed64),
            2 => Ok(Self::LengthDelimited),
            5 => Ok(Self::Fixed32),
            other => Err(DecodeError::InvalidWireType(other)),
        }
    }
}

/// Optional protobuf integer/fixed wire encoding override.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireKind {
    Default,
    Sint32,
    Sint64,
    Fixed32,
    Sfixed32,
    Fixed64,
    Sfixed64,
}

/// Errors encountered while decoding protobuf binary payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnexpectedEof,
    MalformedVarint,
    InvalidWireType(u32),
    WireTypeMismatch {
        expected: WireType,
        actual: WireType,
    },
    InvalidLength,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected EOF while decoding protobuf"),
            Self::MalformedVarint => write!(f, "malformed protobuf varint"),
            Self::InvalidWireType(w) => write!(f, "invalid protobuf wire type {w}"),
            Self::WireTypeMismatch { expected, actual } => {
                write!(
                    f,
                    "wire type mismatch: expected {expected:?}, got {actual:?}"
                )
            }
            Self::InvalidLength => write!(f, "invalid protobuf length prefix"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Trait implemented by generated protobuf enums.
pub trait PbEnum: Copy + Clone + Default + PartialEq + Eq {
    fn from_i32_or_default(value: i32) -> Self;
    fn to_i32(self) -> i32;
}

/// Trait implemented by all generated protobuf message structs.
pub trait PbMessage: Default + Clone {
    /// Encodes all populated fields into `out`.
    fn encode(&self, out: &mut Vec<u8>);

    /// Decodes a single field by `field_number` and `wire_type`.
    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError>;

    /// Merges protobuf binary bytes into `self`.
    fn merge_from(&mut self, bytes: &[u8]) -> Result<(), DecodeError> {
        let mut r = Reader::new(bytes);
        while !r.is_empty() {
            let tag = r.read_varint_u32()?;
            let field_number = tag >> 3;
            let wire_type = WireType::from_u32(tag)?;
            self.decode_field(field_number, wire_type, &mut r)?;
        }
        Ok(())
    }

    /// Decodes a new message instance from `bytes`.
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut msg = Self::default();
        msg.merge_from(bytes)?;
        Ok(msg)
    }

    /// Encodes `self` into a fresh `Vec<u8>`.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode(&mut out);
        out
    }
}

/// Zero-copy slice reader over a protobuf payload.
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Creates a new reader over `buf`.
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns `true` when all bytes have been consumed.
    pub fn is_empty(&self) -> bool {
        self.pos >= self.buf.len()
    }

    /// Reads one byte.
    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let b = *self.buf.get(self.pos).ok_or(DecodeError::UnexpectedEof)?;
        self.pos += 1;
        Ok(b)
    }

    /// Reads an unsigned 64-bit base-128 varint.
    pub fn read_varint_u64(&mut self) -> Result<u64, DecodeError> {
        let mut shift = 0u32;
        let mut result = 0u64;
        loop {
            let b = self.read_u8()?;
            result |= u64::from(b & 0x7F) << shift;
            if (b & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
            if shift >= 64 {
                return Err(DecodeError::MalformedVarint);
            }
        }
    }

    /// Reads a `u32` varint (truncating like standard protobuf).
    pub fn read_varint_u32(&mut self) -> Result<u32, DecodeError> {
        self.read_varint_u64().map(|v| v as u32)
    }

    /// Reads an `i32` varint.
    pub fn read_varint_i32(&mut self) -> Result<i32, DecodeError> {
        self.read_varint_u64().map(|v| v as i32)
    }

    /// Reads an `i64` varint.
    pub fn read_varint_i64(&mut self) -> Result<i64, DecodeError> {
        self.read_varint_u64().map(|v| v as i64)
    }

    /// Reads a zigzag-encoded `sint32`.
    pub fn read_sint32(&mut self) -> Result<i32, DecodeError> {
        let zz = self.read_varint_u32()?;
        Ok(((zz >> 1) as i32) ^ -((zz & 1) as i32))
    }

    /// Reads a zigzag-encoded `sint64`.
    pub fn read_sint64(&mut self) -> Result<i64, DecodeError> {
        let zz = self.read_varint_u64()?;
        Ok(((zz >> 1) as i64) ^ -((zz & 1) as i64))
    }

    /// Reads a `bool` varint.
    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        self.read_varint_u64().map(|v| v != 0)
    }

    /// Reads 4 little-endian bytes.
    pub fn read_fixed32(&mut self) -> Result<u32, DecodeError> {
        let slice = self.take_slice(4)?;
        Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads 8 little-endian bytes.
    pub fn read_fixed64(&mut self) -> Result<u64, DecodeError> {
        let slice = self.take_slice(8)?;
        Ok(u64::from_le_bytes([
            slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
        ]))
    }

    /// Reads a little-endian `f32`.
    pub fn read_f32(&mut self) -> Result<f32, DecodeError> {
        self.read_fixed32().map(f32::from_bits)
    }

    /// Reads a little-endian `f64`.
    pub fn read_f64(&mut self) -> Result<f64, DecodeError> {
        self.read_fixed64().map(f64::from_bits)
    }

    /// Reads an exact sub-slice of `len` bytes.
    pub fn take_slice(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(DecodeError::InvalidLength)?;
        let slice = self
            .buf
            .get(self.pos..end)
            .ok_or(DecodeError::UnexpectedEof)?;
        self.pos = end;
        Ok(slice)
    }

    /// Reads a length-delimited byte slice.
    pub fn read_len_delimited(&mut self) -> Result<&'a [u8], DecodeError> {
        let raw_len = self.read_varint_u64()?;
        let len = usize::try_from(raw_len).map_err(|_| DecodeError::InvalidLength)?;
        self.take_slice(len)
    }

    /// Skips an unknown field based on its [`WireType`].
    pub fn skip_field(&mut self, wire_type: WireType) -> Result<(), DecodeError> {
        match wire_type {
            WireType::Varint => {
                let _ = self.read_varint_u64()?;
            }
            WireType::Fixed64 => {
                let _ = self.take_slice(8)?;
            }
            WireType::LengthDelimited => {
                let _ = self.read_len_delimited()?;
            }
            WireType::Fixed32 => {
                let _ = self.take_slice(4)?;
            }
        }
        Ok(())
    }
}

/// Writes a wire tag `(field_number << 3) | wire_type`.
#[inline]
pub fn write_tag(out: &mut Vec<u8>, field_number: u32, wire_type: WireType) {
    write_varint_u64(out, u64::from((field_number << 3) | (wire_type as u32)));
}

/// Writes an unsigned 64-bit varint.
#[inline]
pub fn write_varint_u64(out: &mut Vec<u8>, mut v: u64) {
    while v >= 0x80 {
        out.push((v as u8 & 0x7F) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

/// Writes a signed 32-bit varint (sign-extended to 64 bits per protobuf spec).
#[inline]
pub fn write_varint_i32(out: &mut Vec<u8>, v: i32) {
    write_varint_u64(out, i64::from(v) as u64);
}

/// Writes a signed 64-bit varint.
#[inline]
pub fn write_varint_i64(out: &mut Vec<u8>, v: i64) {
    write_varint_u64(out, v as u64);
}

/// Writes a zigzag-encoded `sint32`.
#[inline]
pub fn write_sint32(out: &mut Vec<u8>, v: i32) {
    let zz = ((v << 1) ^ (v >> 31)) as u32;
    write_varint_u64(out, u64::from(zz));
}

/// Writes a zigzag-encoded `sint64`.
#[inline]
pub fn write_sint64(out: &mut Vec<u8>, v: i64) {
    let zz = ((v << 1) ^ (v >> 63)) as u64;
    write_varint_u64(out, zz);
}

/// Writes a length-delimited byte slice.
#[inline]
pub fn write_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

/// Writes a nested [`PbMessage`] as a length-delimited field.
#[inline]
pub fn write_submessage<M: PbMessage>(out: &mut Vec<u8>, field_number: u32, msg: &M) {
    let mut tmp = Vec::new();
    msg.encode(&mut tmp);
    write_tag(out, field_number, WireType::LengthDelimited);
    write_bytes(out, &tmp);
}

/// Decodes either a packed repeated scalar field or a single element into `out`.
pub fn decode_repeated_scalar<T, F>(
    wire_type: WireType,
    r: &mut Reader<'_>,
    out: &mut Vec<T>,
    mut read_elem: F,
) -> Result<(), DecodeError>
where
    F: FnMut(&mut Reader<'_>) -> Result<T, DecodeError>,
{
    if wire_type == WireType::LengthDelimited {
        let sub = r.read_len_delimited()?;
        let mut sub_reader = Reader::new(sub);
        while !sub_reader.is_empty() {
            out.push(read_elem(&mut sub_reader)?);
        }
    } else {
        out.push(read_elem(r)?);
    }
    Ok(())
}

/// Helper trait for encoding/decoding protobuf map keys and values cleanly.
pub trait MapCodec: Sized + Default {
    fn wire_type() -> WireType;
    fn is_default(&self) -> bool;
    fn encode_value(&self, out: &mut Vec<u8>);
    fn decode_value(wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError>;
}

impl MapCodec for i32 {
    fn wire_type() -> WireType {
        WireType::Varint
    }
    fn is_default(&self) -> bool {
        *self == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_varint_i32(out, *self);
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_varint_i32()
    }
}

impl MapCodec for i64 {
    fn wire_type() -> WireType {
        WireType::Varint
    }
    fn is_default(&self) -> bool {
        *self == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_varint_i64(out, *self);
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_varint_i64()
    }
}

impl MapCodec for u32 {
    fn wire_type() -> WireType {
        WireType::Varint
    }
    fn is_default(&self) -> bool {
        *self == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_varint_u64(out, u64::from(*self));
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_varint_u32()
    }
}

impl MapCodec for u64 {
    fn wire_type() -> WireType {
        WireType::Varint
    }
    fn is_default(&self) -> bool {
        *self == 0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_varint_u64(out, *self);
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_varint_u64()
    }
}

impl MapCodec for bool {
    fn wire_type() -> WireType {
        WireType::Varint
    }
    fn is_default(&self) -> bool {
        !*self
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_varint_u64(out, u64::from(*self));
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_bool()
    }
}

impl MapCodec for f32 {
    fn wire_type() -> WireType {
        WireType::Fixed32
    }
    fn is_default(&self) -> bool {
        *self == 0.0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_bits().to_le_bytes());
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_f32()
    }
}

impl MapCodec for f64 {
    fn wire_type() -> WireType {
        WireType::Fixed64
    }
    fn is_default(&self) -> bool {
        *self == 0.0
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_bits().to_le_bytes());
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_f64()
    }
}

impl MapCodec for Vec<u8> {
    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
    fn is_default(&self) -> bool {
        self.is_empty()
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        write_bytes(out, self);
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.read_len_delimited().map(|s| s.to_vec())
    }
}

impl<M: PbMessage> MapCodec for M {
    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
    fn is_default(&self) -> bool {
        false
    }
    fn encode_value(&self, out: &mut Vec<u8>) {
        let mut tmp = Vec::new();
        self.encode(&mut tmp);
        write_bytes(out, &tmp);
    }
    fn decode_value(_wire_type: WireType, r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let bytes = r.read_len_delimited()?;
        Self::decode(bytes)
    }
}

/// Encodes all entries of a protobuf map field.
pub fn write_map<K: MapCodec, V: MapCodec>(
    out: &mut Vec<u8>,
    field_number: u32,
    map: &BTreeMap<K, V>,
) {
    for (k, v) in map {
        let mut entry = Vec::new();
        if !k.is_default() {
            write_tag(&mut entry, 1, K::wire_type());
            k.encode_value(&mut entry);
        }
        if !v.is_default() {
            write_tag(&mut entry, 2, V::wire_type());
            v.encode_value(&mut entry);
        }
        write_tag(out, field_number, WireType::LengthDelimited);
        write_bytes(out, &entry);
    }
}

/// Decodes a single protobuf map entry into `map`.
pub fn decode_map_entry<K: MapCodec + Ord, V: MapCodec>(
    r: &mut Reader<'_>,
    map: &mut BTreeMap<K, V>,
) -> Result<(), DecodeError> {
    let sub = r.read_len_delimited()?;
    let mut sub_r = Reader::new(sub);
    let mut key = K::default();
    let mut val = V::default();
    while !sub_r.is_empty() {
        let tag = sub_r.read_varint_u32()?;
        let wt = WireType::from_u32(tag)?;
        match tag >> 3 {
            1 => key = K::decode_value(wt, &mut sub_r)?,
            2 => val = V::decode_value(wt, &mut sub_r)?,
            _ => sub_r.skip_field(wt)?,
        }
    }
    map.insert(key, val);
    Ok(())
}
