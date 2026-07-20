use crate::value::ValueKey;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Display, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[display(
    r#"
Table Index:
    Entries - {:?}
"#,
    entries
)]
pub struct TbIndex {
    /// The structure is `(key_field_value, entry_primary_key)`
    entries: HashSet<(ValueKey, ValueKey)>,
}

impl TbIndex {
    pub fn new(entries: impl Into<HashSet<(ValueKey, ValueKey)>>) -> Self {
        Self {
            entries: entries.into(),
        }
    }

    pub fn entries(&self) -> &HashSet<(ValueKey, ValueKey)> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut HashSet<(ValueKey, ValueKey)> {
        &mut self.entries
    }

    pub fn get_entries(self) -> HashSet<(ValueKey, ValueKey)> {
        self.entries
    }

    pub fn set_entries(&mut self, normal: impl Into<HashSet<(ValueKey, ValueKey)>>) {
        self.entries = normal.into();
    }
}

impl AsRef<Self> for TbIndex {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl From<&TbIndex> for TbIndex {
    fn from(value: &TbIndex) -> Self {
        value.clone()
    }
}
