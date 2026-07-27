use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum VariantType {
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

    Vec,

    F32,
    F64,

    Option,
    HashSet,
    HashMap,
}

impl VariantType {
    pub fn is_indexable(&self) -> bool {
        match self {
            VariantType::String => true,
            VariantType::Bool => true,
            VariantType::Char => true,
            VariantType::InlineBlob => true,
            VariantType::Blob => false,

            VariantType::USize => true,
            VariantType::U8 => true,
            VariantType::U16 => true,
            VariantType::U32 => true,
            VariantType::U64 => true,
            VariantType::U128 => true,

            VariantType::ISize => true,
            VariantType::I8 => true,
            VariantType::I16 => true,
            VariantType::I32 => true,
            VariantType::I64 => true,
            VariantType::I128 => true,

            VariantType::Vec => true,

            VariantType::F32 => false,
            VariantType::F64 => false,

            VariantType::Option => false,
            VariantType::HashSet => false,
            VariantType::HashMap => false,
        }
    }
}

impl AsRef<Self> for VariantType {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&VariantType> for VariantType {
    fn from(value: &VariantType) -> Self {
        value.to_owned()
    }
}
