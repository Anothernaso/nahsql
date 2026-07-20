use super::field::{KeyType, SchemaField};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a table in a database schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaTable {
    pub(super) name: String,
    pub(super) primary_key: String,
    pub(super) fields: HashMap<String, SchemaField>,
}

impl SchemaTable {
    /// Creates a new schema table with
    /// the given name, primary key, and fields.
    ///
    /// # Notes
    ///
    /// At least one field must have its `key_type`
    /// set to `PrimaryKey`. If more than one field
    /// has `key_type` set to `PrimaryKey`, the last
    /// one encountered will be used as the primary key.
    ///
    pub fn new(name: impl Into<String>, fields: impl IntoIterator<Item = SchemaField>) -> Self {
        let mut primary_key: String = String::new();

        let fields = fields
            .into_iter()
            .map(|field| {
                if field.key_type == KeyType::PrimaryKey {
                    primary_key = field.name.clone();
                }
                (field.name.clone(), field)
            })
            .collect();

        Self {
            name: name.into(),
            primary_key,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn primary_key(&self) -> &str {
        &self.primary_key
    }

    pub fn fields(&self) -> &HashMap<String, SchemaField> {
        &self.fields
    }
}
