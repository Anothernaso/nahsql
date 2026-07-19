use crate::value::ValueType;
use serde::{Deserialize, Serialize};

/// Represents a field of a table in a database schema.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaField {
    pub(super) name: String,
    pub(super) is_key: bool,
    pub(super) r#type: ValueType,
}

impl SchemaField {
    /// Creates a new schema field with the given name, key flag, and type.
    ///
    /// # Arguments
    ///
    /// `is_key` is a flag indicating whether the
    /// field is a key field.
    /// A key field must have a `ValueType`
    /// that is compatible with `ValueKey` as
    /// it will be indexed.
    ///
    pub fn new(
        name: impl Into<String>,
        is_key: impl Into<bool>,
        r#type: impl Into<ValueType>,
    ) -> Self {
        Self {
            name: name.into(),
            is_key: is_key.into(),
            r#type: r#type.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_key(&self) -> bool {
        self.is_key
    }

    pub fn r#type(&self) -> ValueType {
        self.r#type
    }
}
