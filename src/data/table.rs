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
    pub entries: HashSet<ValueKey>,
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
