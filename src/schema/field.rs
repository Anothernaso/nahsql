use crate::value::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaField {
    pub(super) name: String,
    pub(super) is_key: bool,
    pub(super) r#type: ValueType,
}

impl SchemaField {
    pub fn new(name: impl Into<String>, is_key: bool, r#type: impl Into<ValueType>) -> Self {
        Self {
            name: name.into(),
            is_key,
            r#type: r#type.into(),
        }
    }
}
