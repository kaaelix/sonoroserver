//! Serde serializer and deserializer bridging `sonoro_common::zon::ZonValue`.

use std::fmt::Display;

use serde::de::{
    self, DeserializeOwned, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};
use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};
use sonoro_common::zon::{self, ZonValue};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZonCodecError {
    #[error("ZON parse error: {0}")]
    Parse(#[from] zon::ZonParseError),
    #[error("{0}")]
    Custom(String),
}

impl de::Error for ZonCodecError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl ser::Error for ZonCodecError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

pub fn from_zon_str<T: DeserializeOwned>(input: &str) -> Result<T, ZonCodecError> {
    let val = zon::parse(input)?;
    from_zon_value(val)
}

pub fn from_zon_value<T: DeserializeOwned>(val: ZonValue) -> Result<T, ZonCodecError> {
    T::deserialize(ZonDe(val))
}

pub fn to_zon_value<T: Serialize>(value: &T) -> Result<ZonValue, ZonCodecError> {
    value.serialize(ZonSer)
}

pub fn to_zon_string<T: Serialize>(value: &T) -> Result<String, ZonCodecError> {
    let zv = to_zon_value(value)?;
    Ok(zon::stringify(&zv))
}

struct ZonDe(ZonValue);

impl<'de> IntoDeserializer<'de, ZonCodecError> for ZonDe {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserializer<'de> for ZonDe {
    type Error = ZonCodecError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Null => visitor.visit_unit(),
            ZonValue::Bool(b) => visitor.visit_bool(b),
            ZonValue::Int(n) => visitor.visit_i64(n),
            ZonValue::Float(f) => visitor.visit_f64(f),
            ZonValue::Str(s) => visitor.visit_string(s),
            ZonValue::EnumLiteral(s) => visitor.visit_string(s),
            ZonValue::Array(arr) => visitor.visit_seq(VecSeq(arr.into_iter())),
            ZonValue::Struct(fields) => visitor.visit_map(FieldMap {
                iter: fields.into_iter(),
                pending_val: None,
            }),
        }
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Float(f) => visitor.visit_f32(f as f32),
            ZonValue::Int(n) => visitor.visit_f32(n as f32),
            other => ZonDe(other).deserialize_any(visitor),
        }
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Float(f) => visitor.visit_f64(f),
            ZonValue::Int(n) => visitor.visit_f64(n as f64),
            other => ZonDe(other).deserialize_any(visitor),
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Null => visitor.visit_none(),
            other => visitor.visit_some(ZonDe(other)),
        }
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Array(arr) => visitor.visit_seq(VecSeq(arr.into_iter())),
            // Empty `.{}` parses as ZonValue::Struct(vec![]).
            ZonValue::Struct(fields) if fields.is_empty() => {
                visitor.visit_seq(VecSeq(Vec::new().into_iter()))
            }
            other => ZonDe(other).deserialize_any(visitor),
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self.0 {
            ZonValue::Struct(fields) => visitor.visit_map(FieldMap {
                iter: fields.into_iter(),
                pending_val: None,
            }),
            ZonValue::Array(arr) if arr.is_empty() => visitor.visit_map(FieldMap {
                iter: Vec::new().into_iter(),
                pending_val: None,
            }),
            other => ZonDe(other).deserialize_any(visitor),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 char str string
        bytes byte_buf unit unit_struct newtype_struct map enum identifier
        ignored_any
    }
}

struct VecSeq(std::vec::IntoIter<ZonValue>);

impl<'de> SeqAccess<'de> for VecSeq {
    type Error = ZonCodecError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        match self.0.next() {
            Some(v) => seed.deserialize(ZonDe(v)).map(Some),
            None => Ok(None),
        }
    }
}

struct FieldMap {
    iter: std::vec::IntoIter<(String, ZonValue)>,
    pending_val: Option<ZonValue>,
}

impl<'de> MapAccess<'de> for FieldMap {
    type Error = ZonCodecError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        match self.iter.next() {
            Some((k, v)) => {
                self.pending_val = Some(v);
                seed.deserialize(MapKeyDe(k)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let val = self
            .pending_val
            .take()
            .ok_or_else(|| ZonCodecError::Custom("missing map value".into()))?;
        seed.deserialize(ZonDe(val))
    }
}

struct MapKeyDe(String);

macro_rules! impl_map_key_parse {
    ($de_method:ident, $visit_method:ident, $ty:ty) => {
        fn $de_method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            let parsed = self.0.parse::<$ty>().map_err(|e| {
                ZonCodecError::Custom(format!(
                    "invalid {} map key '{}': {e}",
                    stringify!($ty),
                    self.0
                ))
            })?;
            visitor.$visit_method(parsed)
        }
    };
}

impl<'de> Deserializer<'de> for MapKeyDe {
    type Error = ZonCodecError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.0)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0.as_str() {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            other => Err(ZonCodecError::Custom(format!(
                "invalid bool map key '{other}'"
            ))),
        }
    }

    impl_map_key_parse!(deserialize_i8, visit_i8, i8);
    impl_map_key_parse!(deserialize_i16, visit_i16, i16);
    impl_map_key_parse!(deserialize_i32, visit_i32, i32);
    impl_map_key_parse!(deserialize_i64, visit_i64, i64);
    impl_map_key_parse!(deserialize_i128, visit_i128, i128);
    impl_map_key_parse!(deserialize_u8, visit_u8, u8);
    impl_map_key_parse!(deserialize_u16, visit_u16, u16);
    impl_map_key_parse!(deserialize_u32, visit_u32, u32);
    impl_map_key_parse!(deserialize_u64, visit_u64, u64);
    impl_map_key_parse!(deserialize_u128, visit_u128, u128);
    impl_map_key_parse!(deserialize_f32, visit_f32, f32);
    impl_map_key_parse!(deserialize_f64, visit_f64, f64);

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.0 == "null" {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_enum(KeyEnumAccess(self.0))
    }

    serde::forward_to_deserialize_any! {
        char str string bytes byte_buf unit unit_struct seq tuple tuple_struct
        map struct identifier ignored_any
    }
}

struct KeyEnumAccess(String);

impl<'de> de::EnumAccess<'de> for KeyEnumAccess {
    type Error = ZonCodecError;
    type Variant = UnitVariantOnly;

    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let val = seed.deserialize(MapKeyDe(self.0))?;
        Ok((val, UnitVariantOnly))
    }
}

struct UnitVariantOnly;

impl<'de> de::VariantAccess<'de> for UnitVariantOnly {
    type Error = ZonCodecError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(
        self,
        _seed: T,
    ) -> Result<T::Value, Self::Error> {
        Err(ZonCodecError::Custom(
            "map enum key cannot be a newtype variant".into(),
        ))
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(ZonCodecError::Custom(
            "map enum key cannot be a tuple variant".into(),
        ))
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(ZonCodecError::Custom(
            "map enum key cannot be a struct variant".into(),
        ))
    }
}

struct ZonSer;

impl Serializer for ZonSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;
    type SerializeSeq = ZonSeqSer;
    type SerializeTuple = ZonSeqSer;
    type SerializeTupleStruct = ZonSeqSer;
    type SerializeTupleVariant = ZonSeqVariantSer;
    type SerializeMap = ZonMapSer;
    type SerializeStruct = ZonStructSer;
    type SerializeStructVariant = ZonStructVariantSer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Int(i64::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        i64::try_from(v)
            .map(ZonValue::Int)
            .map_err(|_| ZonCodecError::Custom(format!("u64 value {v} exceeds i64::MAX")))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Float(f64::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Str(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Str(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Array(
            v.iter().map(|&b| ZonValue::Int(i64::from(b))).collect(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(Vec::new()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::EnumLiteral(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(vec![(
            variant.to_string(),
            value.serialize(Self)?,
        )]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ZonSeqSer(Vec::with_capacity(len.unwrap_or(0))))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ZonSeqSer(Vec::with_capacity(len)))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ZonSeqSer(Vec::with_capacity(len)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(ZonSeqVariantSer {
            variant: variant.to_string(),
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(ZonMapSer {
            fields: Vec::with_capacity(len.unwrap_or(0)),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(ZonStructSer(Vec::with_capacity(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(ZonStructVariantSer {
            variant: variant.to_string(),
            fields: Vec::with_capacity(len),
        })
    }
}

struct ZonSeqSer(Vec<ZonValue>);

impl SerializeSeq for ZonSeqSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.push(value.serialize(ZonSer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Array(self.0))
    }
}

impl SerializeTuple for ZonSeqSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.push(value.serialize(ZonSer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Array(self.0))
    }
}

impl SerializeTupleStruct for ZonSeqSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.0.push(value.serialize(ZonSer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Array(self.0))
    }
}

struct ZonSeqVariantSer {
    variant: String,
    items: Vec<ZonValue>,
}

impl SerializeTupleVariant for ZonSeqVariantSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.items.push(value.serialize(ZonSer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(vec![(
            self.variant,
            ZonValue::Array(self.items),
        )]))
    }
}

struct ZonMapSer {
    fields: Vec<(String, ZonValue)>,
    key: Option<String>,
}

impl SerializeMap for ZonMapSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        match key.serialize(ZonSer)? {
            ZonValue::Str(s) => self.key = Some(s),
            ZonValue::Int(n) => self.key = Some(n.to_string()),
            ZonValue::Bool(b) => self.key = Some(b.to_string()),
            ZonValue::Float(f) => self.key = Some(f.to_string()),
            ZonValue::EnumLiteral(s) => self.key = Some(s),
            other => {
                return Err(ZonCodecError::Custom(format!(
                    "unsupported map key: {}",
                    zon::stringify(&other)
                )));
            }
        }
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let k = self
            .key
            .take()
            .ok_or_else(|| ZonCodecError::Custom("missing map key".into()))?;
        self.fields.push((k, value.serialize(ZonSer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(self.fields))
    }
}

struct ZonStructSer(Vec<(String, ZonValue)>);

impl SerializeStruct for ZonStructSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.0.push((key.to_string(), value.serialize(ZonSer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(self.0))
    }
}

struct ZonStructVariantSer {
    variant: String,
    fields: Vec<(String, ZonValue)>,
}

impl SerializeStructVariant for ZonStructVariantSer {
    type Ok = ZonValue;
    type Error = ZonCodecError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.fields.push((key.to_string(), value.serialize(ZonSer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(ZonValue::Struct(vec![(
            self.variant,
            ZonValue::Struct(self.fields),
        )]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    enum KeyKind {
        Alpha,
        Beta,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct MapHolder {
        by_i32: BTreeMap<i32, String>,
        by_u64: BTreeMap<u64, i32>,
        by_bool: BTreeMap<bool, String>,
        by_enum: BTreeMap<KeyKind, i32>,
    }

    #[test]
    fn roundtrips_integer_bool_and_enum_map_keys() {
        let orig = MapHolder {
            by_i32: BTreeMap::from([(-5, "neg".into()), (42, "pos".into())]),
            by_u64: BTreeMap::from([(0, 10), (i64::MAX as u64, 99)]),
            by_bool: BTreeMap::from([(false, "no".into()), (true, "yes".into())]),
            by_enum: BTreeMap::from([(KeyKind::Alpha, 1), (KeyKind::Beta, 2)]),
        };
        let zon_str = to_zon_string(&orig).expect("serialize maps");
        let decoded: MapHolder = from_zon_str(&zon_str).expect("deserialize maps");
        assert_eq!(decoded, orig);
    }

    #[test]
    fn rejects_u64_exceeding_i64_max() {
        let too_big = (i64::MAX as u64) + 1;
        assert!(to_zon_value(&too_big).is_err());
        assert!(to_zon_value(&u64::MAX).is_err());

        let bad_map = BTreeMap::from([(u64::MAX, "x")]);
        assert!(to_zon_string(&bad_map).is_err());
    }
}
