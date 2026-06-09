use crate::schema::field::SchemaField;
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
    /// # Arguments
    ///
    /// `primary key` is the name of
    /// the field that will serve
    /// as the primary key of the database
    /// and therefore must be a valid
    /// key field in the table.
    ///
    pub fn new(
        name: impl Into<String>,
        primary_key: impl Into<String>,
        fields: impl IntoIterator<Item = SchemaField>,
    ) -> Self {
        Self {
            name: name.into(),
            primary_key: primary_key.into(),
            fields: fields
                .into_iter()
                .map(|field| (field.name.clone(), field))
                .collect(),
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
