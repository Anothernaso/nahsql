use crate::value::ValueType;
use derive_more::Display as MoreDisplay;
use serde::{Deserialize, Serialize};
use strum_macros::Display as StrumDisplay;

/// Represents what type of key a field is.
#[derive(
    Debug, StrumDisplay, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum KeyType {
    /// The field is not a key field.
    NonKey,

    /// The field is a normal key field, (i.e, not unique).
    NormalKey,

    /// The field is a unique key field.
    UniqueKey,

    /// The field is a primary key field which is also unique.
    ///
    /// # Notes
    ///
    /// There may only be one primary key field in a
    /// given table, and if more than one is specified,
    /// then the last one will be used.
    ///
    PrimaryKey,
}

impl AsRef<Self> for KeyType {
    fn as_ref(&self) -> &Self {
        &self
    }
}

/// Represents a field of a table in a database schema.
#[derive(
    Debug, MoreDisplay, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[display(
    r#"
Schema Field:
    Name - {}
    Key Type - {}
    Value Type - {}
"#,
    name,
    key_type,
    value_type
)]
pub struct SchemaField {
    pub(super) name: String,
    pub(super) key_type: KeyType,
    pub(super) value_type: ValueType,
}

impl SchemaField {
    /// Creates a new schema field with the given name, key type, and value type.
    ///
    /// # Arguments
    ///
    /// `name` is the name of the field.
    /// `key_type` is the type of key that the field should be.
    /// `value_type` determines which type of values the field can hold.
    ///
    pub fn new(
        name: impl Into<String>,
        key_type: impl Into<KeyType>,
        value_type: impl Into<ValueType>,
    ) -> Self {
        Self {
            name: name.into(),
            key_type: key_type.into(),
            value_type: value_type.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key_type(&self) -> KeyType {
        self.key_type
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }
}
