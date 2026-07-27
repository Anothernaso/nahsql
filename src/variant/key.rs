use super::Variant;
use super::r#type::VariantType;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KeyVariant {
    String(String),
    Bool(bool),
    Char(char),
    InlineBlob(Vec<u8>),

    USize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    ISize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),

    Vec(Vec<KeyVariant>),
}

impl KeyVariant {
    pub fn r#type(&self) -> VariantType {
        Variant::from(self).r#type()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            KeyVariant::String(v) => v.into_bytes(),
            KeyVariant::InlineBlob(v) => v,
            KeyVariant::Bool(v) => vec![v as u8],
            KeyVariant::Char(v) => {
                let mut buf = [0; 4];
                v.encode_utf8(&mut buf);
                buf.to_vec()
            }

            KeyVariant::U8(v) => v.to_le_bytes().to_vec(),
            KeyVariant::U16(v) => v.to_le_bytes().to_vec(),
            KeyVariant::U32(v) => v.to_le_bytes().to_vec(),
            KeyVariant::U64(v) => v.to_le_bytes().to_vec(),
            KeyVariant::U128(v) => v.to_le_bytes().to_vec(),
            KeyVariant::USize(v) => v.to_le_bytes().to_vec(),

            KeyVariant::I8(v) => v.to_le_bytes().to_vec(),
            KeyVariant::I16(v) => v.to_le_bytes().to_vec(),
            KeyVariant::I32(v) => v.to_le_bytes().to_vec(),
            KeyVariant::I64(v) => v.to_le_bytes().to_vec(),
            KeyVariant::I128(v) => v.to_le_bytes().to_vec(),
            KeyVariant::ISize(v) => v.to_le_bytes().to_vec(),

            KeyVariant::Vec(v) => v.into_iter().flat_map(|v| v.into_bytes()).collect(),
        }
    }

    pub fn into_value(self) -> Variant {
        match self {
            KeyVariant::String(v) => Variant::String(v),
            KeyVariant::Bool(v) => Variant::Bool(v),
            KeyVariant::Char(v) => Variant::Char(v),
            KeyVariant::InlineBlob(v) => Variant::InlineBlob(v),

            KeyVariant::USize(v) => Variant::USize(v),
            KeyVariant::U8(v) => Variant::U8(v),
            KeyVariant::U16(v) => Variant::U16(v),
            KeyVariant::U32(v) => Variant::U32(v),
            KeyVariant::U64(v) => Variant::U64(v),
            KeyVariant::U128(v) => Variant::U128(v),

            KeyVariant::ISize(v) => Variant::ISize(v),
            KeyVariant::I8(v) => Variant::I8(v),
            KeyVariant::I16(v) => Variant::I16(v),
            KeyVariant::I32(v) => Variant::I32(v),
            KeyVariant::I64(v) => Variant::I64(v),
            KeyVariant::I128(v) => Variant::I128(v),

            KeyVariant::Vec(v) => Variant::Vec(v.into_iter().map(|v| v.into_value()).collect()),
        }
    }
}

impl From<KeyVariant> for Vec<u8> {
    fn from(key: KeyVariant) -> Self {
        key.into_bytes()
    }
}

impl From<Variant> for Option<KeyVariant> {
    fn from(value: Variant) -> Self {
        value.into_key()
    }
}

impl AsRef<Self> for KeyVariant {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&KeyVariant> for KeyVariant {
    fn from(key: &KeyVariant) -> Self {
        key.to_owned()
    }
}
