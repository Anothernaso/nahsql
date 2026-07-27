use super::Value;
use super::r#type::ValueType;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ValueKey {
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

impl ValueKey {
    pub fn r#type(&self) -> ValueType {
        Value::from(self).r#type()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            ValueKey::String(s) => s.into_bytes(),
            ValueKey::InlineBlob(b) => b,
            ValueKey::Bool(b) => vec![b as u8],
            ValueKey::Char(c) => {
                let mut buf = [0; 4];
                c.encode_utf8(&mut buf);
                buf.to_vec()
            }

            ValueKey::U8(v) => v.to_le_bytes().to_vec(),
            ValueKey::U16(v) => v.to_le_bytes().to_vec(),
            ValueKey::U32(v) => v.to_le_bytes().to_vec(),
            ValueKey::U64(v) => v.to_le_bytes().to_vec(),
            ValueKey::U128(v) => v.to_le_bytes().to_vec(),
            ValueKey::USize(v) => v.to_le_bytes().to_vec(),

            ValueKey::I8(v) => v.to_le_bytes().to_vec(),
            ValueKey::I16(v) => v.to_le_bytes().to_vec(),
            ValueKey::I32(v) => v.to_le_bytes().to_vec(),
            ValueKey::I64(v) => v.to_le_bytes().to_vec(),
            ValueKey::I128(v) => v.to_le_bytes().to_vec(),
            ValueKey::ISize(v) => v.to_le_bytes().to_vec(),
        }
    }

    pub fn into_value(self) -> Value {
        match self {
            ValueKey::String(value) => Value::String(value),
            ValueKey::Bool(value) => Value::Bool(value),
            ValueKey::Char(value) => Value::Char(value),
            ValueKey::InlineBlob(value) => Value::InlineBlob(value),

            ValueKey::USize(value) => Value::USize(value),
            ValueKey::U8(value) => Value::U8(value),
            ValueKey::U16(value) => Value::U16(value),
            ValueKey::U32(value) => Value::U32(value),
            ValueKey::U64(value) => Value::U64(value),
            ValueKey::U128(value) => Value::U128(value),

            ValueKey::ISize(value) => Value::ISize(value),
            ValueKey::I8(value) => Value::I8(value),
            ValueKey::I16(value) => Value::I16(value),
            ValueKey::I32(value) => Value::I32(value),
            ValueKey::I64(value) => Value::I64(value),
            ValueKey::I128(value) => Value::I128(value),
        }
    }
}

impl From<ValueKey> for Vec<u8> {
    fn from(key: ValueKey) -> Self {
        key.into_bytes()
    }
}

impl From<Value> for Option<ValueKey> {
    fn from(value: Value) -> Self {
        value.into_key()
    }
}

impl AsRef<Self> for ValueKey {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&ValueKey> for ValueKey {
    fn from(key: &ValueKey) -> Self {
        key.to_owned()
    }
}
