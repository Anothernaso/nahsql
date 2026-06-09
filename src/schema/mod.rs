mod field;
mod table;

pub use field::SchemaField;
pub use table::SchemaTable;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub(self) version: u64,
    pub(self) tables: HashMap<String, SchemaTable>,
}

impl Schema {
    pub fn new(version: u64, tables: impl IntoIterator<Item = SchemaTable>) -> Self {
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

    pub fn version(&self) -> u64 {
        self.version
    }
}
