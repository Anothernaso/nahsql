mod key;
mod r#type;

pub use key::*;
pub use r#type::*;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum_macros::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum Variant {
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

    Vec(Vec<Variant>),

    F32(f32),
    F64(f64),

    Option(Option<Box<Variant>>),
    HashSet(HashSet<KeyVariant>),

    /// WARNING: This variant CAN NOT be serialized
    /// because most `serde` serializers don't support
    /// `HashMap`s with non-`String` keys and will cause runtime errors;
    /// therefore this variant is not supported by the database.
    HashMap(HashMap<KeyVariant, Variant>),
}

impl Variant {
    pub fn r#type(&self) -> VariantType {
        match *self {
            Self::String(_) => VariantType::String,
            Self::Bool(_) => VariantType::Bool,
            Self::Char(_) => VariantType::Char,
            Self::InlineBlob(_) => VariantType::InlineBlob,
            Self::Blob => VariantType::Blob,

            Self::USize(_) => VariantType::USize,
            Self::U8(_) => VariantType::U8,
            Self::U16(_) => VariantType::U16,
            Self::U32(_) => VariantType::U32,
            Self::U64(_) => VariantType::U64,
            Self::U128(_) => VariantType::U128,

            Self::ISize(_) => VariantType::ISize,
            Self::I8(_) => VariantType::I8,
            Self::I16(_) => VariantType::I16,
            Self::I32(_) => VariantType::I32,
            Self::I64(_) => VariantType::I64,
            Self::I128(_) => VariantType::I128,

            Self::Vec(_) => VariantType::Vec,

            Self::F32(_) => VariantType::F32,
            Self::F64(_) => VariantType::F64,

            Self::Option(_) => VariantType::Option,
            Self::HashSet(_) => VariantType::HashSet,
            Self::HashMap(_) => VariantType::HashMap,
        }
    }

    pub fn into_key(self) -> Option<KeyVariant> {
        match self {
            Variant::String(value) => Some(KeyVariant::String(value)),
            Variant::Bool(value) => Some(KeyVariant::Bool(value)),
            Variant::Char(value) => Some(KeyVariant::Char(value)),
            Variant::InlineBlob(value) => Some(KeyVariant::InlineBlob(value)),
            Variant::Blob => None,

            Variant::USize(value) => Some(KeyVariant::USize(value)),
            Variant::U8(value) => Some(KeyVariant::U8(value)),
            Variant::U16(value) => Some(KeyVariant::U16(value)),
            Variant::U32(value) => Some(KeyVariant::U32(value)),
            Variant::U64(value) => Some(KeyVariant::U64(value)),
            Variant::U128(value) => Some(KeyVariant::U128(value)),

            Variant::ISize(value) => Some(KeyVariant::ISize(value)),
            Variant::I8(value) => Some(KeyVariant::I8(value)),
            Variant::I16(value) => Some(KeyVariant::I16(value)),
            Variant::I32(value) => Some(KeyVariant::I32(value)),
            Variant::I64(value) => Some(KeyVariant::I64(value)),
            Variant::I128(value) => Some(KeyVariant::I128(value)),

            Variant::Vec(_) => None,

            Variant::F32(_) => None,
            Variant::F64(_) => None,

            Variant::Option(_) => None,
            Variant::HashSet(_) => None,
            Variant::HashMap(_) => None,
        }
    }
}

impl From<KeyVariant> for Variant {
    fn from(key: KeyVariant) -> Self {
        key.into_value()
    }
}

impl From<&KeyVariant> for Variant {
    fn from(key: &KeyVariant) -> Self {
        key.to_owned().into_value()
    }
}

impl AsRef<Self> for Variant {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&Variant> for Variant {
    fn from(value: &Variant) -> Self {
        value.clone()
    }
}
