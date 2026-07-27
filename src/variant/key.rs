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
}

impl KeyVariant {
    pub fn r#type(&self) -> VariantType {
        Variant::from(self).r#type()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            KeyVariant::String(s) => s.into_bytes(),
            KeyVariant::InlineBlob(b) => b,
            KeyVariant::Bool(b) => vec![b as u8],
            KeyVariant::Char(c) => {
                let mut buf = [0; 4];
                c.encode_utf8(&mut buf);
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
        }
    }

    pub fn into_value(self) -> Variant {
        match self {
            KeyVariant::String(value) => Variant::String(value),
            KeyVariant::Bool(value) => Variant::Bool(value),
            KeyVariant::Char(value) => Variant::Char(value),
            KeyVariant::InlineBlob(value) => Variant::InlineBlob(value),

            KeyVariant::USize(value) => Variant::USize(value),
            KeyVariant::U8(value) => Variant::U8(value),
            KeyVariant::U16(value) => Variant::U16(value),
            KeyVariant::U32(value) => Variant::U32(value),
            KeyVariant::U64(value) => Variant::U64(value),
            KeyVariant::U128(value) => Variant::U128(value),

            KeyVariant::ISize(value) => Variant::ISize(value),
            KeyVariant::I8(value) => Variant::I8(value),
            KeyVariant::I16(value) => Variant::I16(value),
            KeyVariant::I32(value) => Variant::I32(value),
            KeyVariant::I64(value) => Variant::I64(value),
            KeyVariant::I128(value) => Variant::I128(value),
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
