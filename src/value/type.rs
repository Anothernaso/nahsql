use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

impl AsRef<Self> for ValueType {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&ValueType> for ValueType {
    fn from(value: &ValueType) -> Self {
        value.to_owned()
    }
}
