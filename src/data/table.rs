use crate::variant::KeyVariant;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Display, Default, Clone, Serialize, Deserialize)]
#[display(
    r#"
Database Table:
    Entries - {:?}
"#,
    entries
)]
pub struct DbTable {
    /// List of all primary keys in the table
    entries: HashSet<KeyVariant>,
}

impl DbTable {
    pub fn new(entries: impl Into<HashSet<KeyVariant>>) -> Self {
        Self {
            entries: entries.into(),
        }
    }

    pub fn entries(&self) -> &HashSet<KeyVariant> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut HashSet<KeyVariant> {
        &mut self.entries
    }

    pub fn get_entries(self) -> HashSet<KeyVariant> {
        self.entries
    }

    pub fn set_entries(&mut self, entries: impl Into<HashSet<KeyVariant>>) {
        self.entries = entries.into();
    }
}

impl AsRef<Self> for DbTable {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&DbTable> for DbTable {
    fn from(value: &DbTable) -> Self {
        value.clone()
    }
}
