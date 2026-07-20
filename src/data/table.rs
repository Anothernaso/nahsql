use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Database Table:
    Entries - {:?}
"#,
    entries
)]
pub struct DbTable {
    /// List of all primary keys in the table
    entries: HashSet<ValueKey>,
}

impl DbTable {
    pub fn new(entries: impl Into<HashSet<ValueKey>>) -> Self {
        Self {
            entries: entries.into(),
        }
    }

    pub fn entries(&self) -> &HashSet<ValueKey> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut HashSet<ValueKey> {
        &mut self.entries
    }

    pub fn set_entries(&mut self, entries: impl Into<HashSet<ValueKey>>) {
        self.entries = entries.into();
    }
}

impl AsRef<Self> for DbTable {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Into<DbTable> for &DbTable {
    fn into(self) -> DbTable {
        self.clone()
    }
}
