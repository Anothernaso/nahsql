//! Database schema and
//! related data structures.

mod error;
mod field;
mod table;

pub use error::Error;
pub use field::SchemaField;
pub use table::SchemaTable;

use crate::meta::SchemaVersion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the entire database schema.
///
/// It defines the structure of a database,
/// its tables, the fields within those tables,
/// the schema version and so on.
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub(self) version: SchemaVersion,
    pub(self) tables: HashMap<String, SchemaTable>,
}

impl Schema {
    /// Creates a new schema with the given version and tables.
    pub fn new(version: SchemaVersion, tables: impl IntoIterator<Item = SchemaTable>) -> Self {
        Self {
            version,
            tables: tables
                .into_iter()
                .map(|field| (field.name.clone(), field))
                .collect(),
        }
    }

    pub fn tables(&self) -> &HashMap<String, SchemaTable> {
        &self.tables
    }

    pub fn version(&self) -> SchemaVersion {
        self.version
    }
}
