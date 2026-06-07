use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Bool(bool),
    Char(char),
    InlineBlob(Vec<u8>),
    Blob,

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

    F32(f32),
    F64(f64),
}

impl Value {
    pub fn r#type(&self) -> ValueType {
        match *self {
            Self::String(_) => ValueType::String,
            Self::Bool(_) => ValueType::Bool,
            Self::Char(_) => ValueType::Char,
            Self::InlineBlob(_) => ValueType::InlineBlob,
            Self::Blob => ValueType::Blob,

            Self::USize(_) => ValueType::USize,
            Self::U8(_) => ValueType::U8,
            Self::U16(_) => ValueType::U16,
            Self::U32(_) => ValueType::U32,
            Self::U64(_) => ValueType::U64,
            Self::U128(_) => ValueType::U128,

            Self::ISize(_) => ValueType::ISize,
            Self::I8(_) => ValueType::I8,
            Self::I16(_) => ValueType::I16,
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::I128(_) => ValueType::I128,

            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
        }
    }
}

impl From<ValueKey> for Value {
    fn from(value: ValueKey) -> Self {
        match value {
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

impl From<&ValueKey> for Value {
    fn from(value: &ValueKey) -> Self {
        match value {
            ValueKey::String(value) => Value::String(value.to_owned()),
            ValueKey::Bool(value) => Value::Bool(value.to_owned()),
            ValueKey::Char(value) => Value::Char(value.to_owned()),
            ValueKey::InlineBlob(value) => Value::InlineBlob(value.to_owned()),

            ValueKey::USize(value) => Value::USize(value.to_owned()),
            ValueKey::U8(value) => Value::U8(value.to_owned()),
            ValueKey::U16(value) => Value::U16(value.to_owned()),
            ValueKey::U32(value) => Value::U32(value.to_owned()),
            ValueKey::U64(value) => Value::U64(value.to_owned()),
            ValueKey::U128(value) => Value::U128(value.to_owned()),

            ValueKey::ISize(value) => Value::ISize(value.to_owned()),
            ValueKey::I8(value) => Value::I8(value.to_owned()),
            ValueKey::I16(value) => Value::I16(value.to_owned()),
            ValueKey::I32(value) => Value::I32(value.to_owned()),
            ValueKey::I64(value) => Value::I64(value.to_owned()),
            ValueKey::I128(value) => Value::I128(value.to_owned()),
        }
    }
}

impl From<Value> for Option<ValueKey> {
    fn from(value: Value) -> Option<ValueKey> {
        match value {
            Value::String(value) => Some(ValueKey::String(value)),
            Value::Bool(value) => Some(ValueKey::Bool(value)),
            Value::Char(value) => Some(ValueKey::Char(value)),
            Value::InlineBlob(value) => Some(ValueKey::InlineBlob(value)),
            Value::Blob => None,

            Value::USize(value) => Some(ValueKey::USize(value)),
            Value::U8(value) => Some(ValueKey::U8(value)),
            Value::U16(value) => Some(ValueKey::U16(value)),
            Value::U32(value) => Some(ValueKey::U32(value)),
            Value::U64(value) => Some(ValueKey::U64(value)),
            Value::U128(value) => Some(ValueKey::U128(value)),

            Value::ISize(value) => Some(ValueKey::ISize(value)),
            Value::I8(value) => Some(ValueKey::I8(value)),
            Value::I16(value) => Some(ValueKey::I16(value)),
            Value::I32(value) => Some(ValueKey::I32(value)),
            Value::I64(value) => Some(ValueKey::I64(value)),
            Value::I128(value) => Some(ValueKey::I128(value)),

            Value::F32(_) => None,
            Value::F64(_) => None,
        }
    }
}

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
}

impl From<ValueKey> for Vec<u8> {
    fn from(key: ValueKey) -> Vec<u8> {
        match key {
            ValueKey::String(s) => s.into_bytes(),
            ValueKey::InlineBlob(b) => b,
            ValueKey::Bool(b) => vec![b as u8],
            ValueKey::Char(c) => {
                let mut buf = [0; 4];
                c.encode_utf8(&mut buf);
                buf.to_vec()
            }

            ValueKey::U8(v) => vec![v],
            ValueKey::U16(v) => v.to_le_bytes().to_vec(),
            ValueKey::U32(v) => v.to_le_bytes().to_vec(),
            ValueKey::U64(v) => v.to_le_bytes().to_vec(),
            ValueKey::U128(v) => v.to_le_bytes().to_vec(),
            ValueKey::USize(v) => v.to_le_bytes().to_vec(),

            ValueKey::I8(v) => vec![v as u8],
            ValueKey::I16(v) => v.to_le_bytes().to_vec(),
            ValueKey::I32(v) => v.to_le_bytes().to_vec(),
            ValueKey::I64(v) => v.to_le_bytes().to_vec(),
            ValueKey::I128(v) => v.to_le_bytes().to_vec(),
            ValueKey::ISize(v) => v.to_le_bytes().to_vec(),
        }
    }
}

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ValueType {
    String,
    Bool,
    Char,
    InlineBlob,
    Blob,

    USize,
    U8,
    U16,
    U32,
    U64,
    U128,

    ISize,
    I8,
    I16,
    I32,
    I64,
    I128,

    F32,
    F64,
}

impl ValueType {
    pub fn is_indexable(&self) -> bool {
        match self {
            ValueType::String => true,
            ValueType::Bool => true,
            ValueType::Char => true,
            ValueType::InlineBlob => true,
            ValueType::Blob => false,

            ValueType::USize => true,
            ValueType::U8 => true,
            ValueType::U16 => true,
            ValueType::U32 => true,
            ValueType::U64 => true,
            ValueType::U128 => true,

            ValueType::ISize => true,
            ValueType::I8 => true,
            ValueType::I16 => true,
            ValueType::I32 => true,
            ValueType::I64 => true,
            ValueType::I128 => true,

            ValueType::F32 => false,
            ValueType::F64 => false,
        }
    }
}
