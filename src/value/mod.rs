mod key;
mod r#type;

pub use key::*;
pub use r#type::*;

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

    pub fn into_key(self) -> Option<ValueKey> {
        match self {
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

impl From<ValueKey> for Value {
    fn from(key: ValueKey) -> Self {
        key.into_value()
    }
}

impl From<&ValueKey> for Value {
    fn from(key: &ValueKey) -> Self {
        key.to_owned().into_value()
    }
}

impl AsRef<Self> for Value {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
    }
}
